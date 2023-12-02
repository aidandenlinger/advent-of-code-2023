alias n := new
alias c := continue

new DAY:
	cp -r .template {{DAY}}a
	# Rename the project to the day
	sd template {{DAY}}a {{DAY}}a/Cargo.toml
	# Get the input
	mv ~/Downloads/input input/{{DAY}}.txt
	# Use the input file
	sd template {{DAY}} {{DAY}}a/src/main.rs

continue DAY:
	cp -r {{DAY}}a {{DAY}}b
	sd {{DAY}}a {{DAY}}b {{DAY}}b/Cargo.toml

runall:
	for x in day*; do cd $x; cargo run --release; cd ..; done
