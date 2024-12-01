build:
	find . -name "day*" -d 1 -type d -exec echo "build {}" \; -exec sh -c "cd {} && cargo build --verbose; cd .." \;

test:
	find . -name "day*" -d 1 -type d -exec echo "test {}" \; -exec sh -c "cd {} && cargo test --verbose; cd .." \;
