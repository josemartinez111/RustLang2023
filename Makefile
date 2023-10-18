# Makefile
# ___________________________________________________________

# Recipe that uses the positional argument
watch:
	@echo "Running watch for $(PROJECT)"
	@cd $(PROJECT) && cargo watch -q -c -w src/ -x run | lolcat
# ___________________________________________________________

test:
	@echo "Running watch for tests in $(PROJECT)"
	@cd $(PROJECT) && cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture" | lolcat
# ___________________________________________________________

# Add package to project with features
#add:
#	@echo "Adding package $(PACKAGE) to project $(PROJECT_NAME) with feature(s) $(FEATURE)"
#	cargo add $(PACKAGE) -p $(PROJECT_NAME) --features "$(FEATURE)"
# ___________________________________________________________





