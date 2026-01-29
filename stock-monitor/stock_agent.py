#!/usr/bin/env python3
"""
Stock Monitoring Agent
Demonstrates: data fetching, processing, alerting, scheduling

Dependencies: pip install yfinance
"""

import yfinance as yf
from datetime import datetime
from dataclasses import dataclass
from typing import Callable


# ============================================================
# CONFIGURATION
# ============================================================

WATCHLIST = [
    {"symbol": "AAPL", "alert_below": 150.00, "alert_above": 250.00},
    {"symbol": "MSFT", "alert_below": 350.00, "alert_above": 500.00},
    {"symbol": "GOOGL", "alert_below": 140.00, "alert_above": 200.00},
]


# ============================================================
# DATA STRUCTURES
# ============================================================

@dataclass
class StockQuote:
    symbol: str
    price: float
    change: float
    change_pct: float
    volume: int
    timestamp: datetime


@dataclass
class Alert:
    symbol: str
    message: str
    price: float
    threshold: float


# ============================================================
# DATA FETCHING
# ============================================================

def fetch_quote(symbol: str) -> StockQuote | None:
    """Fetch current quote from Yahoo Finance"""
    try:
        ticker = yf.Ticker(symbol)
        info = ticker.info
        hist = ticker.history(period="2d")

        if hist.empty:
            return None

        current = hist['Close'].iloc[-1]
        previous = hist['Close'].iloc[-2] if len(hist) > 1 else current
        change = current - previous
        change_pct = (change / previous) * 100 if previous else 0

        return StockQuote(
            symbol=symbol,
            price=round(current, 2),
            change=round(change, 2),
            change_pct=round(change_pct, 2),
            volume=int(hist['Volume'].iloc[-1]),
            timestamp=datetime.now()
        )
    except Exception as e:
        print(f"Error fetching {symbol}: {e}")
        return None


def fetch_all(symbols: list[str]) -> list[StockQuote]:
    """Fetch quotes for all symbols"""
    quotes = []
    for symbol in symbols:
        quote = fetch_quote(symbol)
        if quote:
            quotes.append(quote)
    return quotes


# ============================================================
# ANALYSIS / ALERTS
# ============================================================

def check_alerts(quote: StockQuote, config: dict) -> list[Alert]:
    """Check if quote triggers any alert conditions"""
    alerts = []

    if "alert_below" in config and quote.price < config["alert_below"]:
        alerts.append(Alert(
            symbol=quote.symbol,
            message=f"BELOW threshold",
            price=quote.price,
            threshold=config["alert_below"]
        ))

    if "alert_above" in config and quote.price > config["alert_above"]:
        alerts.append(Alert(
            symbol=quote.symbol,
            message=f"ABOVE threshold",
            price=quote.price,
            threshold=config["alert_above"]
        ))

    return alerts


def analyze_watchlist(watchlist: list[dict]) -> tuple[list[StockQuote], list[Alert]]:
    """Fetch and analyze entire watchlist"""
    all_quotes = []
    all_alerts = []

    for item in watchlist:
        quote = fetch_quote(item["symbol"])
        if quote:
            all_quotes.append(quote)
            alerts = check_alerts(quote, item)
            all_alerts.extend(alerts)

    return all_quotes, all_alerts


# ============================================================
# OUTPUT / NOTIFICATION
# ============================================================

def format_report(quotes: list[StockQuote], alerts: list[Alert]) -> str:
    """Generate text report"""
    lines = []
    lines.append(f"\n{'='*60}")
    lines.append(f"STOCK MONITOR REPORT - {datetime.now().strftime('%Y-%m-%d %H:%M')}")
    lines.append(f"{'='*60}\n")

    # Quotes table
    lines.append(f"{'Symbol':<8} {'Price':>10} {'Change':>10} {'%':>8} {'Volume':>12}")
    lines.append("-" * 52)

    for q in quotes:
        sign = "+" if q.change >= 0 else ""
        lines.append(
            f"{q.symbol:<8} {q.price:>10.2f} {sign}{q.change:>9.2f} "
            f"{sign}{q.change_pct:>7.2f}% {q.volume:>12,}"
        )

    # Alerts
    if alerts:
        lines.append(f"\n{'!'*60}")
        lines.append("ALERTS:")
        for a in alerts:
            lines.append(f"  {a.symbol}: {a.message} - Price: ${a.price:.2f} (Threshold: ${a.threshold:.2f})")
        lines.append(f"{'!'*60}")

    return "\n".join(lines)


def notify(message: str, method: str = "console"):
    """Send notification via specified method"""
    if method == "console":
        print(message)
    elif method == "file":
        with open("stock_report.txt", "a") as f:
            f.write(message + "\n")
    # Extend: email, SMS, webhook, etc.


# ============================================================
# AGENT CORE
# ============================================================

class StockAgent:
    """
    Core agent class - encapsulates monitoring logic
    Can be extended with custom analyzers and notifiers
    """

    def __init__(self, watchlist: list[dict]):
        self.watchlist = watchlist
        self.analyzers: list[Callable] = [check_alerts]
        self.notifiers: list[Callable] = [notify]

    def add_analyzer(self, func: Callable):
        """Add custom analysis function"""
        self.analyzers.append(func)

    def add_notifier(self, func: Callable):
        """Add custom notification handler"""
        self.notifiers.append(func)

    def run_once(self):
        """Execute single monitoring cycle"""
        quotes, alerts = analyze_watchlist(self.watchlist)
        report = format_report(quotes, alerts)

        for notifier in self.notifiers:
            notifier(report)

        return quotes, alerts

    def run_continuous(self, interval_seconds: int = 3600):
        """Run continuously with specified interval"""
        import time

        print(f"Starting continuous monitoring (interval: {interval_seconds}s)")
        print("Press Ctrl+C to stop\n")

        while True:
            try:
                self.run_once()
                time.sleep(interval_seconds)
            except KeyboardInterrupt:
                print("\nMonitoring stopped.")
                break


# ============================================================
# ENTRY POINT
# ============================================================

if __name__ == "__main__":
    import sys

    agent = StockAgent(WATCHLIST)

    if len(sys.argv) > 1 and sys.argv[1] == "--continuous":
        # Run every hour
        agent.run_continuous(interval_seconds=3600)
    else:
        # Single run
        agent.run_once()
