# Progress Records

This document lists the development achievements for the cTrader FIX API library

- Base FixApi implementation :white_check_mark:
- Base requests :white_check_mark:
- Example code :white_check_mark:
    - Connect :white_check_mark:
    - Send logon :white_check_mark:
    - Send logout :white_check_mark:
    - Disconnect :white_check_mark:
- Handle responses :white_check_mark:
    - Implement response structure :white_check_mark:
    - Implement response handler - notify :white_check_mark:
- Add Error struct using `thiserror` :white_check_mark:
- MarketClient :white_check_mark:
    - Internal Market data Callback :white_check_mark:
    - Parsing response message :white_check_mark:
    - Subscribe the symbol for spot :white_check_mark:
    - Implement the check the request has accepted method :white_check_mark:
    - Test for parsing market datas :white_check_mark:
    - Unsubscribe the symbol for spot :white_check_mark:
    - Subscribe the symbol for depth :white_check_mark:
    - Unsubscribe the symbol for depth :white_check_mark:
    - Parsing the spot market data in callback :white_check_mark:
    - Add quote spot data method :white_check_mark:
    - Parsing the depth market data in callback :white_check_mark:
    - Parsing the incremental market data in callback :white_check_mark:
    - Market data handler in example code :white_check_mark:
    - Fix callback method for subscription :white_check_mark:
- FIXED identify with message type and id :white_check_mark:
- FIXED the issue of heartbeat :white_check_mark:
- TradeClient :white_check_mark:
    - Add fetch methods :white_check_mark:
    - Implement fetch_security_list to fetch the security list :white_check_mark:
    - Implement fetch_positions :white_check_mark:
    - Implement fetch_all_order_status :white_check_mark:
    - Implement new_market_order :white_check_mark:
    - Implement new_limit_order :white_check_mark:
    - Implement new_stop_order :white_check_mark:
    - Implement parse_func for ExecutionReport :white_check_mark:
    - Implement cancel_order :white_check_mark:
    - Implement replace_order :white_check_mark:
    - Implement adjust_position_size :white_check_mark:
    - Implement close_position :white_check_mark:
    - Added timeout in request methods :white_check_mark:
    - FIXED issue unhandled trade message (deadlock) :white_check_mark:
    - Add handler for trade execution :white_check_mark:
    - FIXED data parsing issue in Socket :white_check_mark:
	- Removed unnecessary arguments for new order methods :white_check_mark:
- Added support for ResendRequest :white_check_mark:
- Fixed issue - removed the heartbeat task from `TradeClient`. (provider send the recurring HB at the interval) :white_check_mark: