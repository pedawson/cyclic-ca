# Stock Monitor Agent
## Mini Manual

---

## 1. Overview

A Python-based agent that monitors stock prices and generates alerts when thresholds are crossed.

```
┌────────────┐    ┌────────────┐    ┌────────────┐    ┌────────────┐
│ WATCHLIST  │ →  │   FETCH    │ →  │  ANALYZE   │ →  │   NOTIFY   │
│  (config)  │    │   (API)    │    │  (alerts)  │    │  (output)  │
└────────────┘    └────────────┘    └────────────┘    └────────────┘
```

---

## 2. Installation

```
COMMAND                         DESCRIPTION
─────────────────────────────────────────────────────────────────────
cd /path/to/stock-monitor       Change to project directory
pip install yfinance            Install Yahoo Finance API library
```

---

## 3. Configuration

Edit `WATCHLIST` in `stock_agent.py`:

```
FIELD           TYPE      DESCRIPTION
─────────────────────────────────────────────────────────────────────
symbol          string    Stock ticker symbol (e.g., "AAPL")
alert_below     float     Alert when price falls below this value
alert_above     float     Alert when price rises above this value
```

Example:
```python
WATCHLIST = [
    {"symbol": "AAPL",  "alert_below": 150.00, "alert_above": 250.00},
    {"symbol": "MSFT",  "alert_below": 350.00, "alert_above": 500.00},
    {"symbol": "GOOGL", "alert_below": 140.00, "alert_above": 200.00},
]
```

---

## 4. Usage Commands

```
COMMAND                              DESCRIPTION
─────────────────────────────────────────────────────────────────────
python stock_agent.py                Run once, output to console
python stock_agent.py --continuous   Run continuously (hourly)
python scheduler.py --schedule       Run Python-based scheduler
python scheduler.py --install-launchd Generate macOS launchd plist
```

---

## 5. Component Reference

```
COMPONENT           FILE              FUNCTION
─────────────────────────────────────────────────────────────────────
Configuration       stock_agent.py    WATCHLIST variable
Data Structure      stock_agent.py    StockQuote dataclass
Alert Structure     stock_agent.py    Alert dataclass
Data Fetcher        stock_agent.py    fetch_quote()
Alert Checker       stock_agent.py    check_alerts()
Report Generator    stock_agent.py    format_report()
Output Handler      stock_agent.py    notify()
Agent Core          stock_agent.py    StockAgent class
Scheduler           scheduler.py      cron / schedule / launchd
```

---

## 6. Data Structures

### StockQuote
```
FIELD           TYPE        DESCRIPTION
─────────────────────────────────────────────────────────────────────
symbol          str         Ticker symbol
price           float       Current price
change          float       Dollar change from previous close
change_pct      float       Percentage change
volume          int         Trading volume
timestamp       datetime    Time of fetch
```

### Alert
```
FIELD           TYPE        DESCRIPTION
─────────────────────────────────────────────────────────────────────
symbol          str         Ticker symbol
message         str         Alert description
price           float       Current price
threshold       float       Threshold that was crossed
```

---

## 7. Execution Flow

```
STEP    ACTION                          OUTPUT
─────────────────────────────────────────────────────────────────────
1       Load WATCHLIST                  List of stock configs
2       For each symbol:
2a        Call fetch_quote()            StockQuote object
2b        Call check_alerts()           List of Alert objects
3       Aggregate all quotes/alerts     Combined lists
4       Call format_report()            Formatted text string
5       Call notify()                   Output to console/file
```

---

## 8. Scheduling Options

### Option A: Cron (Linux/macOS)
```
CRON EXPRESSION                 DESCRIPTION
─────────────────────────────────────────────────────────────────────
30 9 * * 1-5                    9:30 AM, Monday-Friday
0 16 * * 1-5                    4:00 PM, Monday-Friday
```

Edit crontab:
```
crontab -e
```

Add line:
```
30 9 * * 1-5 /usr/bin/python3 /path/to/stock_agent.py >> stock.log 2>&1
```

### Option B: Python schedule library
```
pip install schedule
python scheduler.py --schedule
```
Runs in foreground. Use with screen/tmux for persistence.

### Option C: macOS launchd
```
python scheduler.py --install-launchd
launchctl load ~/Library/LaunchAgents/com.user.stockmonitor.plist
```

Management:
```
COMMAND                                              ACTION
─────────────────────────────────────────────────────────────────────
launchctl load ~/Library/LaunchAgents/com...plist   Start service
launchctl unload ~/Library/LaunchAgents/com...plist Stop service
launchctl list | grep stockmonitor                  Check status
```

---

## 9. Extension Points

```
EXTENSION           METHOD                  EXAMPLE USE
─────────────────────────────────────────────────────────────────────
Custom analyzer     agent.add_analyzer(fn)  Moving averages, RSI
Custom notifier     agent.add_notifier(fn)  Email, SMS, Slack
Data source         Replace fetch_quote()   Alpha Vantage, Polygon
State persistence   Add load/save methods   Track daily changes
```

---

## 10. File Manifest

```
FILE                LINES   PURPOSE
─────────────────────────────────────────────────────────────────────
stock_agent.py      ~150    Core agent logic
scheduler.py        ~80     Scheduling utilities
README.md           ~100    Architecture documentation
MANUAL.md           ---     This reference
```

---

## 11. Quick Reference Card

```
─────────────────────────────────────────────────────────────────────
RUN ONCE            python stock_agent.py
RUN CONTINUOUS      python stock_agent.py --continuous
EDIT WATCHLIST      Edit WATCHLIST in stock_agent.py
ADD ANALYZER        agent.add_analyzer(your_function)
ADD NOTIFIER        agent.add_notifier(your_function)
SCHEDULE (CRON)     crontab -e → add entry
SCHEDULE (MACOS)    python scheduler.py --install-launchd
VIEW LOGS           cat stock.log
─────────────────────────────────────────────────────────────────────
```
