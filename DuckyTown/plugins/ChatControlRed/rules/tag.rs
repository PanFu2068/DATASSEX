# -----------------------------------------------------------------------------------------------
# This file manages rules applying for player tags: nick, prefix and suffix
# and includes rules from global.rs.
#
# Permissions required for color usage:
# chatcontrol.color.{color_name}
# chatcontrol.hexcolor.{color_name}
#
# To fine tune control over color usage in nicknames, 
# prefixes or suffixes, see the rule examples below.
# -----------------------------------------------------------------------------------------------

@import global

# Prevent certain words being used as nicks.
#match Notch
#require tag nick
#strip colors true
#then warn {prefix_error}This nickname is not allowed!
#then deny

# User cannot use color codes in their nicks unless they have the bypass perms
#match (§|&)(#[a-f0-9]{6}|[a-f0-9k-or])
#require tag nick
#strip colors false
#ignore perm chatcontrol.nick.colors
#then warn {prefix_error}You can not use colors in your nickname!
#then deny

# User can only change color codes in their nicks unless they have the bypass perms
#match (.*)
#require tag nick
#strip colors true
#require script "$0" != "{player}"
#ignore perm chatcontrol.nick.change
#then warn {prefix_error}Your nickname must match your username!
#then deny

# Prevent players writing variables to their tags.
match [({|%)]([^{}]+)[(}|%)]
then warn {prefix_error}You cannot use variables in your tag!
then deny
