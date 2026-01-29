# Stock Monitor Agent

Learning example demonstrating agent architecture patterns.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        AGENT                                │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Data Source │→ │  Analyzer   │→ │     Notifier        │  │
│  │  (yfinance) │  │  (alerts)   │  │ (console/file/etc)  │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          ↑
                    ┌─────────────┐
                    │  Scheduler  │
                    │ (cron/launchd)
                    └─────────────┘
```

## Components

| Component | Purpose | Extensibility |
|-----------|---------|---------------|
| `fetch_quote()` | Data acquisition | Swap API (Alpha Vantage, Polygon, etc.) |
| `check_alerts()` | Analysis logic | Add technical indicators, ML models |
| `notify()` | Output/alerting | Email, SMS, Slack, webhook |
| `StockAgent` | Orchestration | Add state, persistence, backoff |
| `scheduler.py` | Timing | Cron, launchd, or Python-native |

## Usage

```bash
# Install dependency
pip install yfinance

# Single run
python stock_agent.py

# Continuous (hourly)
python stock_agent.py --continuous

# Python scheduler (runs in foreground)
python scheduler.py --schedule

# Install macOS launchd job
python scheduler.py --install-launchd
```

## Configuration

Edit `WATCHLIST` in `stock_agent.py`:

```python
WATCHLIST = [
    {"symbol": "AAPL", "alert_below": 150.00, "alert_above": 250.00},
    {"symbol": "TSLA", "alert_below": 200.00, "alert_above": 400.00},
]
```

## Extension Points

### Add email notifications

```python
import smtplib
from email.mime.text import MIMEText

def notify_email(message: str):
    msg = MIMEText(message)
    msg['Subject'] = 'Stock Alert'
    msg['From'] = 'agent@example.com'
    msg['To'] = 'you@example.com'

    with smtplib.SMTP('smtp.example.com', 587) as s:
        s.starttls()
        s.login('user', 'pass')
        s.send_message(msg)

# Register with agent
agent.add_notifier(notify_email)
```

### Add technical analysis

```python
def check_moving_average(quote, config):
    """Alert when price crosses 50-day MA"""
    ticker = yf.Ticker(quote.symbol)
    hist = ticker.history(period="60d")
    ma50 = hist['Close'].rolling(50).mean().iloc[-1]

    alerts = []
    if quote.price > ma50 and hist['Close'].iloc[-2] < ma50:
        alerts.append(Alert(quote.symbol, "Crossed ABOVE 50-day MA", quote.price, ma50))
    return alerts

agent.add_analyzer(check_moving_average)
```

### Persist state (track daily changes)

```python
import json
from pathlib import Path

STATE_FILE = Path("agent_state.json")

def load_state():
    if STATE_FILE.exists():
        return json.loads(STATE_FILE.read_text())
    return {}

def save_state(state):
    STATE_FILE.write_text(json.dumps(state, indent=2))
```

## Key Patterns Demonstrated

1. **Separation of concerns** - Fetch, analyze, notify are independent
2. **Plugin architecture** - Add analyzers/notifiers without modifying core
3. **Configuration-driven** - Watchlist is data, not code
4. **Multiple scheduling options** - Choose based on deployment environment
5. **Graceful degradation** - Errors in one stock don't crash entire run
