#!/usr/bin/env python3
"""
Scheduling Options for Stock Agent

Three approaches shown:
1. Cron (system-level, most reliable for daily)
2. schedule library (Python-native)
3. launchd (macOS native)
"""

# ============================================================
# OPTION 1: CRON
# ============================================================
#
# Edit crontab: crontab -e
#
# Run daily at 9:30 AM (market open):
# 30 9 * * 1-5 /usr/bin/python3 /path/to/stock_agent.py >> /path/to/stock.log 2>&1
#
# Format: minute hour day month weekday command
# 1-5 = Monday through Friday
#


# ============================================================
# OPTION 2: schedule library
# ============================================================
#
# pip install schedule

def run_with_schedule():
    """Python-based scheduling"""
    import schedule
    import time
    from stock_agent import StockAgent, WATCHLIST

    agent = StockAgent(WATCHLIST)

    # Schedule daily at market open
    schedule.every().monday.at("09:30").do(agent.run_once)
    schedule.every().tuesday.at("09:30").do(agent.run_once)
    schedule.every().wednesday.at("09:30").do(agent.run_once)
    schedule.every().thursday.at("09:30").do(agent.run_once)
    schedule.every().friday.at("09:30").do(agent.run_once)

    # Also run at market close
    schedule.every().monday.at("16:00").do(agent.run_once)
    schedule.every().tuesday.at("16:00").do(agent.run_once)
    schedule.every().wednesday.at("16:00").do(agent.run_once)
    schedule.every().thursday.at("16:00").do(agent.run_once)
    schedule.every().friday.at("16:00").do(agent.run_once)

    print("Scheduler running. Press Ctrl+C to stop.")
    while True:
        schedule.run_pending()
        time.sleep(60)


# ============================================================
# OPTION 3: LAUNCHD (macOS)
# ============================================================
#
# Create: ~/Library/LaunchAgents/com.user.stockmonitor.plist
#
LAUNCHD_PLIST = """<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.user.stockmonitor</string>

    <key>ProgramArguments</key>
    <array>
        <string>/usr/bin/python3</string>
        <string>/Users/pauldawson/Projects/Claude/stock-monitor/stock_agent.py</string>
    </array>

    <key>StartCalendarInterval</key>
    <array>
        <!-- Monday-Friday at 9:30 AM -->
        <dict>
            <key>Weekday</key><integer>1</integer>
            <key>Hour</key><integer>9</integer>
            <key>Minute</key><integer>30</integer>
        </dict>
        <dict>
            <key>Weekday</key><integer>2</integer>
            <key>Hour</key><integer>9</integer>
            <key>Minute</key><integer>30</integer>
        </dict>
        <dict>
            <key>Weekday</key><integer>3</integer>
            <key>Hour</key><integer>9</integer>
            <key>Minute</key><integer>30</integer>
        </dict>
        <dict>
            <key>Weekday</key><integer>4</integer>
            <key>Hour</key><integer>9</integer>
            <key>Minute</key><integer>30</integer>
        </dict>
        <dict>
            <key>Weekday</key><integer>5</integer>
            <key>Hour</key><integer>9</integer>
            <key>Minute</key><integer>30</integer>
        </dict>
    </array>

    <key>StandardOutPath</key>
    <string>/Users/pauldawson/Projects/Claude/stock-monitor/stock.log</string>

    <key>StandardErrorPath</key>
    <string>/Users/pauldawson/Projects/Claude/stock-monitor/stock_error.log</string>
</dict>
</plist>
"""

def install_launchd():
    """Generate and install launchd plist"""
    plist_path = "~/Library/LaunchAgents/com.user.stockmonitor.plist"
    expanded = plist_path.replace("~", "/Users/pauldawson")

    with open(expanded, "w") as f:
        f.write(LAUNCHD_PLIST)

    print(f"Created: {plist_path}")
    print(f"Load with: launchctl load {plist_path}")
    print(f"Unload with: launchctl unload {plist_path}")


if __name__ == "__main__":
    import sys

    if len(sys.argv) > 1:
        if sys.argv[1] == "--schedule":
            run_with_schedule()
        elif sys.argv[1] == "--install-launchd":
            install_launchd()
    else:
        print("Usage:")
        print("  --schedule        Run Python scheduler")
        print("  --install-launchd Generate macOS launchd plist")
