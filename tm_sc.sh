#!/bin/bash

s_name="server-client"

tmux new-session -d -s $s_name
tmux rename-window -t 1 "editor"
tmux new-window -t $s_name:2 -n "server"
tmux new-window -t $s_name:3 -n "client"
tmux attach
tmux send-keys -t C-a 1