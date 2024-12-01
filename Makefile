build:
	find . -name "day*" -type d -depth 1 -exec echo "build {}" \; -exec sh -c "cd {} && cargo build --verbose; cd .." \;

test:
	find . -name "day*" -type d -depth 1-exec echo "test {}" \; -exec sh -c "cd {} && cargo test --verbose; cd .." \;
