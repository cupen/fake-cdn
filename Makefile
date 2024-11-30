
start:
	tmux new-session -d -s tablec
	tmux split-window -t "tablec:0"   -v
	tmux send-keys -t "tablec:0.0" "cargo run web --listen 127.0.0.1:9520 --token helloworld" Enter
	tmux send-keys -t "tablec:0.1" "pytest ./" Enter
	tmux select-pane -t "tablec:0.1"
	tmux attach -t tablec
	make stop


stop:
	sleep 0.1
	tmux kill-session -t tablec || true


