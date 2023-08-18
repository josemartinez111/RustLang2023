# JustFile
# ___________________________________________________________

# Recipe that uses the positional argument
watch ARG_HERE:
    cargo watch -x "run -p {{ARG_HERE}}"
# ___________________________________________________________

#add PACKAGE PROJECT FEATURE:
#    cargo add -p {{PROJECT}} {{PACKAGE}} --features {{FEATURE}}
add PACKAGE PROJECT_NAME FEATURE="":
    @echo "Adding package {{PACKAGE}} to project {{PROJECT_NAME}} with feature(s) {{FEATURE}}"
    cargo add {{PACKAGE}} -p {{PROJECT_NAME}} --features "{{FEATURE}}"

# ___________________________________________________________