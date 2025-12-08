
sed -i 's|edition = "2018"|edition = "2021"|g' Cargo.toml
sed -i 's|cortex-m = "0.7.2"|cortex-m = { git = "https://github.com/Sonair-AS/cortex-m.git", branch = "certified_subset", package = "cortex-m" }|g' Cargo.toml
sed -i 's|bare-metal = "1.0.0"|bare-metal = { git = "https://github.com/Sonair-AS/bare-metal.git", branch = "certified_subset" }|g' Cargo.toml
sed -i 's|version = ">=0.6.15,<0.8"|git = "https://github.com/Sonair-AS/cortex-m.git"\nbranch = "certified_subset"\npackage = "cortex-m-rt"|g' Cargo.toml

CHECK_LINE="certified_subset = \["

CERTIFIED_FEATURES='
impl-debug = []

certified_subset = [
    "bare-metal/certified_subset",
    "cortex-m/certified_subset",
    "cortex-m-rt/certified_subset",
]

[lints.rust]
mismatched_lifetime_syntaxes = "allow"
'

# Check if we already added the new features
if ! grep -q "$CHECK_LINE" Cargo.toml; then
    # Append the extra features to the end of the file
    echo "$CERTIFIED_FEATURES" >> Cargo.toml
fi

find ./src -type f | xargs -P 8 -I '{}' sed -i 's/\(#\[derive(.*\)Debug, /#[cfg_attr(feature = "impl-debug", derive(Debug))]\n\1/g' {}
find ./src -type f -name mod.rs | xargs -P 8  -I '{}' sed -i 's/\(impl.* core::fmt\)/#[cfg(feature = "impl-debug")]\n\1/g' {}
