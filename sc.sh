#!/bin/bash

s_name="server-client"

#if [ "$TERM_PROGRAM" = tmux ]; then


#tmux new-session -d -s $s_name
tmux rename-session $s_name
tmux rename-window "scc"
tmux send-keys "cd server && cargo run" C-a
tmux send-keys Enter

tmux split-window -h 
tmux send-keys "cd client && cargo run" C-a
tmux send-keys Enter

#tmux split-window -v
#tmux send-keys "cd client && cargo run" C-a
#tmux send-keys Enter