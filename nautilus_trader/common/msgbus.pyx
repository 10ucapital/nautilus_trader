# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2023 Nautech Systems Pty Ltd. All rights reserved.
#  https://nautechsystems.io
#
#  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
#  You may not use this file except in compliance with the License.
#  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.
# -------------------------------------------------------------------------------------------------

import copy
from typing import Any
from typing import Callable

import cython
import msgspec
import numpy as np

cimport numpy as np

from nautilus_trader.common.clock cimport Clock
from nautilus_trader.common.logging cimport Logger
from nautilus_trader.core.correctness cimport Condition
from nautilus_trader.core.rust.common cimport msgbus_drop
from nautilus_trader.core.rust.common cimport msgbus_new
from nautilus_trader.core.rust.common cimport msgbus_publish_external
from nautilus_trader.core.string cimport pybytes_to_cstr
from nautilus_trader.core.string cimport pystr_to_cstr
from nautilus_trader.core.uuid cimport UUID4
from nautilus_trader.model.identifiers cimport TraderId
from nautilus_trader.serialization.base cimport EXTERNAL_PUBLISHING_TYPES
from nautilus_trader.serialization.base cimport Serializer


cdef class MessageBus:
    """
    Provides a generic message bus to facilitate various messaging patterns.

    The bus provides both a producer and consumer API for Pub/Sub, Req/Rep, as
    well as direct point-to-point messaging to registered endpoints.

    Pub/Sub wildcard patterns for hierarchical topics are possible:
     - `*` asterisk represents one or more characters in a pattern.
     - `?` question mark represents a single character in a pattern.

    Given a topic and pattern potentially containing wildcard characters, i.e.
    `*` and `?`, where `?` can match any single character in the topic, and `*`
    can match any number of characters including zero characters.

    The asterisk in a wildcard matches any character zero or more times. For
    example, `comp*` matches anything beginning with `comp` which means `comp`,
    `complete`, and `computer` are all matched.

    A question mark matches a single character once. For example, `c?mp` matches
    `camp` and `comp`. The question mark can also be used more than once.
    For example, `c??p` would match both of the above examples and `coop`.

    Parameters
    ----------
    trader_id : TraderId
        The trader ID associated with the message bus.
    clock : Clock
        The clock for the message bus.
    logger : Logger
        The logger for the message bus.
    name : str, optional
        The custom name for the message bus.
    serializer : Serializer, optional
        The serializer for database operations.
    config : MessageBusConfig, optional
        The configuration for the message bus.

    Raises
    ------
    ValueError
        If `name` is not ``None`` and not a valid string.

    Warnings
    --------
    This message bus is not thread-safe and must be called from the same thread
    as the event loop.
    """

    def __init__(
        self,
        TraderId trader_id not None,
        Clock clock not None,
        Logger logger not None,
        UUID4 instance_id = None,
        str name = None,
        Serializer serializer = None,
        config: Any | None = None,
    ):
        # Temporary fix for import error
        from nautilus_trader.config.common import MessageBusConfig

        if instance_id is None:
            instance_id = UUID4()
        if name is None:
            name = type(self).__name__
        Condition.valid_string(name, "name")
        if config is None:
            config = MessageBusConfig()
        Condition.type(config, MessageBusConfig, "config")

        self.trader_id = trader_id

        # Copy and clear `types_filter` before passing down to the core MessageBus
        cdef list types_filter = copy.copy(config.types_filter)
        if config.types_filter is not None:
            config.types_filter.clear()

        self._mem = msgbus_new(
            pystr_to_cstr(trader_id.value),
            pystr_to_cstr(name) if name else NULL,
            pystr_to_cstr(instance_id.to_str()),
            pybytes_to_cstr(msgspec.json.encode(config)),
        )

        self._serializer = serializer
        self._clock = clock
        self._log = LoggerAdapter(component_name=name, logger=logger)

        self._endpoints: dict[str, Callable[[Any], None]] = {}
        self._patterns: dict[str, Subscription[:]] = {}
        self._subscriptions: dict[Subscription, list[str]] = {}
        self._correlation_index: dict[UUID4, Callable[[Any], None]] = {}
        self._has_backing = config.database is not None
        self._publishable_types = EXTERNAL_PUBLISHING_TYPES
        if types_filter is not None:
            self._publishable_types = tuple(o for o in EXTERNAL_PUBLISHING_TYPES if o not in types_filter)

        # Counters
        self.sent_count = 0
        self.req_count = 0
        self.res_count = 0
        self.pub_count = 0

    def __del__(self) -> None:
        if self._mem._0 != NULL:
            msgbus_drop(self._mem)

    cpdef list endpoints(self):
        """
        Return all endpoint addresses registered with the message bus.

        Returns
        -------
        list[str]

        """
        return list(self._endpoints.keys())

    cpdef list topics(self):
        """
        Return all topics with active subscribers.

        Returns
        -------
        list[str]

        """
        return sorted(set([s.topic for s in self._subscriptions.keys()]))

    cpdef list subscriptions(self, str pattern = None):
        """
        Return all subscriptions matching the given topic `pattern`.

        Parameters
        ----------
        pattern : str, optional
            The topic pattern filter. May include wildcard characters `*` and `?`.
            If ``None`` then query is for **all** topics.

        Returns
        -------
        list[Subscription]

        """
        if pattern is None:
            pattern = "*"  # Wildcard
        Condition.valid_string(pattern, "pattern")

        return [s for s in self._subscriptions if is_matching(s.topic, pattern)]

    cpdef bint has_subscribers(self, str pattern = None):
        """
        If the message bus has subscribers for the give topic `pattern`.

        Parameters
        ----------
        pattern : str, optional
            The topic filter. May include wildcard characters `*` and `?`.
            If ``None`` then query is for **all** topics.

        Returns
        -------
        bool

        """
        return len(self.subscriptions(pattern)) > 0

    cpdef bint is_subscribed(self, str topic, handler: Callable[[Any], None]):
        """
        Return if topic and handler is subscribed to the message bus.

        Does not consider any previous `priority`.

        Parameters
        ----------
        topic : str
            The topic of the subscription.
        handler : Callable[[Any], None]
            The handler of the subscription.

        Returns
        -------
        bool

        """
        Condition.valid_string(topic, "topic")
        Condition.callable(handler, "handler")

        # Create subscription
        cdef Subscription sub = Subscription(
            topic=topic,
            handler=handler,
        )

        return sub in self._subscriptions

    cpdef bint is_pending_request(self, UUID4 request_id):
        """
        Return if the given `request_id` is still pending a response.

        Parameters
        ----------
        request_id : UUID4
            The request ID to check (to match the correlation_id).

        Returns
        -------
        bool

        """
        Condition.not_none(request_id, "request_id")

        return request_id in self._correlation_index

    cpdef void register(self, str endpoint, handler: Callable[[Any], None]):
        """
        Register the given `handler` to receive messages at the `endpoint` address.

        Parameters
        ----------
        endpoint : str
            The endpoint address to register.
        handler : Callable[[Any], None]
            The handler for the registration.

        Raises
        ------
        ValueError
            If `endpoint` is not a valid string.
        ValueError
            If `handler` is not of type `Callable`.
        KeyError
            If `endpoint` already registered.

        """
        Condition.valid_string(endpoint, "endpoint")
        Condition.callable(handler, "handler")
        Condition.not_in(endpoint, self._endpoints, "endpoint", "_endpoints")

        self._endpoints[endpoint] = handler

        self._log.debug(f"Added endpoint '{endpoint}' {handler}.")

    cpdef void deregister(self, str endpoint, handler: Callable[[Any], None]):
        """
        Deregister the given `handler` from the `endpoint` address.

        Parameters
        ----------
        endpoint : str
            The endpoint address to deregister.
        handler : Callable[[Any], None]
            The handler to deregister.

        Raises
        ------
        ValueError
            If `endpoint` is not a valid string.
        ValueError
            If `handler` is not of type `Callable`.
        KeyError
            If `endpoint` is not registered.
        ValueError
            If `handler` is not registered at the endpoint.

        """
        Condition.valid_string(endpoint, "endpoint")
        Condition.callable(handler, "handler")
        Condition.is_in(endpoint, self._endpoints, "endpoint", "self._endpoints")
        Condition.equal(handler, self._endpoints[endpoint], "handler", "self._endpoints[endpoint]")

        del self._endpoints[endpoint]

        self._log.debug(f"Removed endpoint '{endpoint}' {handler}.")

    cpdef void send(self, str endpoint, msg: Any):
        """
        Send the given message to the given `endpoint` address.

        Parameters
        ----------
        endpoint : str
            The endpoint address to send the message to.
        msg : object
            The message to send.

        """
        Condition.not_none(endpoint, "endpoint")
        Condition.not_none(msg, "msg")

        handler = self._endpoints.get(endpoint)
        if handler is None:
            self._log.error(
                f"Cannot send message: no endpoint registered at '{endpoint}'.",
            )
            return  # Cannot send

        handler(msg)
        self.sent_count += 1

    cpdef void request(self, str endpoint, Request request):
        """
        Handle the given `request`.

        Will log an error if the correlation ID already exists.

        Parameters
        ----------
        endpoint : str
            The endpoint address to send the request to.
        request : Request
            The request to handle.

        """
        Condition.not_none(endpoint, "endpoint")
        Condition.not_none(request, "request")

        if request.id in self._correlation_index:
            self._log.error(
                f"Cannot handle request: "
                f"duplicate ID {request.id} found in correlation index.",
            )
            return  # Do not handle duplicates

        self._correlation_index[request.id] = request.callback

        handler = self._endpoints.get(endpoint)
        if handler is None:
            self._log.error(
                f"Cannot handle request: no endpoint registered at '{endpoint}'.",
            )
            return  # Cannot handle

        handler(request)
        self.req_count += 1

    cpdef void response(self, Response response):
        """
        Handle the given `response`.

        Will log an error if the correlation ID is not found.

        Parameters
        ----------
        response : Response
            The response to handle

        """
        Condition.not_none(response, "response")

        callback = self._correlation_index.pop(response.correlation_id, None)
        if callback is None:
            self._log.error(
                f"Cannot handle response: "
                f"callback not found for correlation_id {response.correlation_id}.",
            )
            return  # Cannot handle

        callback(response)
        self.res_count += 1

    cpdef void subscribe(
        self,
        str topic,
        handler: Callable[[Any], None],
        int priority = 0,
    ):
        """
        Subscribe to the given message `topic` with the given callback `handler`.

        Parameters
        ----------
        topic : str
            The topic for the subscription. May include wildcard characters
            `*` and `?`.
        handler : Callable[[Any], None]
            The handler for the subscription.
        priority : int, optional
            The priority for the subscription. Determines the ordering of
            handlers receiving messages being processed, higher priority
            handlers will receive messages prior to lower priority handlers.

        Raises
        ------
        ValueError
            If `topic` is not a valid string.
        ValueError
            If `handler` is not of type `Callable`.

        Warnings
        --------
        Assigning priority handling is an advanced feature which *shouldn't
        normally be needed by most users*. **Only assign a higher priority to the
        subscription if you are certain of what you're doing**. If an inappropriate
        priority is assigned then the handler may receive messages before core
        system components have been able to process necessary calculations and
        produce potential side effects for logically sound behavior.

        """
        Condition.valid_string(topic, "topic")
        Condition.callable(handler, "handler")

        # Create subscription
        cdef Subscription sub = Subscription(
            topic=topic,
            handler=handler,
            priority=priority,
        )

        # Check if already exists
        if sub in self._subscriptions:
            self._log.debug(f"{sub} already exists.")
            return

        cdef list matches = []
        cdef list patterns = list(self._patterns.keys())

        cdef str pattern
        cdef list subs
        for pattern in patterns:
            if is_matching(topic, pattern):
                subs = list(self._patterns[pattern])
                subs.append(sub)
                subs = sorted(subs, reverse=True)
                self._patterns[pattern] = np.ascontiguousarray(subs, dtype=Subscription)
                matches.append(pattern)

        self._subscriptions[sub] = sorted(matches)

        self._log.debug(f"Added {sub}.")

    cpdef void unsubscribe(self, str topic, handler: Callable[[Any], None]):
        """
        Unsubscribe the given callback `handler` from the given message `topic`.

        Parameters
        ----------
        topic : str, optional
            The topic to unsubscribe from. May include wildcard characters `*`
            and `?`.
        handler : Callable[[Any], None]
            The handler for the subscription.

        Raises
        ------
        ValueError
            If `topic` is not a valid string.
        ValueError
            If `handler` is not of type `Callable`.

        """
        Condition.valid_string(topic, "topic")
        Condition.callable(handler, "handler")

        cdef Subscription sub = Subscription(topic=topic, handler=handler)

        cdef list patterns = self._subscriptions.get(sub)

        # Check if exists
        if patterns is None:
            self._log.warning(f"{sub} not found.")
            return

        cdef str pattern
        for pattern in patterns:
            subs = list(self._patterns[pattern])
            subs.remove(sub)
            subs = sorted(subs, reverse=True)
            self._patterns[pattern] = np.ascontiguousarray(subs, dtype=Subscription)

        del self._subscriptions[sub]

        self._log.debug(f"Removed {sub}.")

    cpdef void publish(self, str topic, msg: Any):
        """
        Publish the given message for the given `topic`.

        Subscription handlers will receive the message in priority order
        (highest first).

        Parameters
        ----------
        topic : str
            The topic to publish on.
        msg : object
            The message to publish.

        """
        self.publish_c(topic, msg)

    @cython.boundscheck(False)
    @cython.wraparound(False)
    cdef void publish_c(self, str topic, msg: Any):
        Condition.not_none(topic, "topic")
        Condition.not_none(msg, "msg")

        # Get all subscriptions matching topic pattern
        cdef Subscription[:] subs = self._patterns.get(topic)
        if subs is None:
            # Add the topic pattern and get matching subscribers
            subs = self._resolve_subscriptions(topic)

        # Send message to all matched subscribers
        cdef:
            int i
            Subscription sub
        for i in range(len(subs)):
            sub = subs[i]
            sub.handler(msg)

        # Publish externally (if configured)
        cdef bytes payload_bytes
        if self._has_backing and self._serializer is not None:
            if isinstance(msg, self._publishable_types):
                payload_bytes = self._serializer.serialize(msg)
                msgbus_publish_external(
                    &self._mem,
                    pystr_to_cstr(topic),
                    pybytes_to_cstr(payload_bytes),
                )

        self.pub_count += 1

    cdef Subscription[:] _resolve_subscriptions(self, str topic):
        cdef list subs_list = []
        cdef Subscription existing_sub
        for existing_sub in self._subscriptions:
            if is_matching(topic, existing_sub.topic):
                subs_list.append(existing_sub)

        subs_list = sorted(subs_list, reverse=True)
        cdef Subscription[:] subs_array = np.ascontiguousarray(subs_list, dtype=Subscription)
        self._patterns[topic] = subs_array

        cdef list matches
        for sub in subs_array:
            matches = self._subscriptions.get(sub, [])
            if topic not in matches:
                matches.append(topic)
            self._subscriptions[sub] = sorted(matches)

        return subs_array


cdef inline bint is_matching(str topic, str pattern):
    # Get length of string and wildcard pattern
    cdef int n = len(topic)
    cdef int m = len(pattern)

    # Create a DP lookup table
    cdef np.ndarray[np.int8_t, ndim=2] t = np.empty((n + 1, m + 1), dtype=np.int8)
    t.fill(False)

    # If both pattern and string are empty: match
    t[0, 0] = True

    # Handle empty string case (i == 0)
    cdef int j
    for j in range(1, m + 1):
        if pattern[j - 1] == '*':
            t[0, j] = t[0, j - 1]

    # Build a matrix in a bottom-up manner
    cdef int i
    for i in range(1, n + 1):
        for j in range(1, m + 1):
            if pattern[j - 1] == '*':
                t[i, j] = t[i - 1, j] or t[i, j - 1]
            elif pattern[j - 1] == '?' or topic[i - 1] == pattern[j - 1]:
                t[i, j] = t[i - 1, j - 1]

    return t[n, m]


# Python wrapper for test access
def is_matching_py(str topic, str pattern) -> bool:
    return is_matching(topic, pattern)


cdef class Subscription:
    """
    Represents a subscription to a particular topic.

    This is an internal class intended to be used by the message bus to organize
    topics and their subscribers.

    Parameters
    ----------
    topic : str
        The topic for the subscription. May include wildcard characters `*` and `?`.
    handler : Callable[[Message], None]
        The handler for the subscription.
    priority : int
        The priority for the subscription.

    Raises
    ------
    ValueError
        If `topic` is not a valid string.
    ValueError
        If `handler` is not of type `Callable`.
    ValueError
        If `priority` is negative (< 0).

    Notes
    -----
    The subscription equality is determined by the topic and handler,
    priority is not considered (and could change).
    """

    def __init__(
        self,
        str topic,
        handler not None: Callable[[Any], None],
        int priority=0,
    ):
        Condition.valid_string(topic, "topic")
        Condition.callable(handler, "handler")
        Condition.not_negative_int(priority, "priority")

        self.topic = topic
        self.handler = handler
        self.priority = priority

    def __eq__(self, Subscription other) -> bool:
        return self.topic == other.topic and self.handler == other.handler

    def __lt__(self, Subscription other) -> bool:
        return self.priority < other.priority

    def __le__(self, Subscription other) -> bool:
        return self.priority <= other.priority

    def __gt__(self, Subscription other) -> bool:
        return self.priority > other.priority

    def __ge__(self, Subscription other) -> bool:
        return self.priority >= other.priority

    def __hash__(self) -> int:
        # Convert handler to string to avoid builtin_function_or_method hashing issues
        return hash((self.topic, str(self.handler)))

    def __repr__(self) -> str:
        return (
            f"{type(self).__name__}("
            f"topic={self.topic}, "
            f"handler={self.handler}, "
            f"priority={self.priority})"
        )
