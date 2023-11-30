alias n := new

new DAY:
	cp -r .template {{DAY}}
	# Rename the project to the day
	sd template {{DAY}} {{DAY}}/Cargo.toml
	# Get the input
	mv ~/Downloads/input {{DAY}}/input.txt
