alias n := new
alias c := continue

new DAY:
	cp -r .template {{DAY}}a
	# Rename the project to the day
	sd template {{DAY}}a {{DAY}}a/Cargo.toml
	# Get the input
	mv ~/Downloads/input {{DAY}}a/input.txt

continue DAY:
	cp -r {{DAY}}a {{DAY}}b
	sd {{DAY}}a {{DAY}}b {{DAY}}b/Cargo.toml
