#!/bin/bash
Path=$(dirname "$0")
SESSION="cargo_bins"

# Kill session if already exists
tmux kill-session -t $SESSION 2>/dev/null

# Start session, window, and base pane
tmux new-session -d -s $SESSION -n main -c $Path

# Create all 4 panes
tmux split-window -h -t $SESSION -c $Path      # Pane 1
tmux select-pane -t $SESSION:0.0
tmux split-window -v -t $SESSION -c $Path       # Pane 2
tmux select-pane -t $SESSION:0.1
tmux split-window -v -t $SESSION -c $Path       # Pane 3

# Let tmux stabilize before sending commands
sleep 1

# Now send commands to each pane
tmux send-keys -t $SESSION:0.0 "cargo run --bin xpub_proxy" C-m
tmux send-keys -t $SESSION:0.1 "cargo run --bin pub_with_id 2" C-m
tmux send-keys -t $SESSION:0.2 "cargo run --bin serial_pub" C-m
tmux send-keys -t $SESSION:0.3 "cargo run --bin pub_with_id 1" C-m

# Arrange layout and attach
tmux select-layout -t $SESSION tiled
tmux attach-session -t $SESSION
