build:
	find . -name "day*" -type d -exec echo "build {}" \; -exec sh -c "cd {} && cargo build --verbose; cd .." \;

test:
	find . -name "day*" -type d -exec echo "test {}" \; -exec sh -c "cd {} && cargo test --verbose; cd .." \;
