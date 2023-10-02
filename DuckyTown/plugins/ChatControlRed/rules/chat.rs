# -----------------------------------------------------------------------------------------------
# This file applies rules to chat messages and includes rules from global.rs.
#
# For help, see https://github.com/kangarko/ChatControl-Red/wiki/Rules
# -----------------------------------------------------------------------------------------------

@import global

# -----------------------------------------------------------------------------------------------
# Anti spam.
# -----------------------------------------------------------------------------------------------

# Limiting repeating characters to four repeats maximum.
#match (.)(?=([a-z])\1{3})
#then replace

# Prevent special, unicode characters. Those are misused to bypass filters.
# Please keep in mind that non-English languages use them, so only uncomment this on English servers.
#match [^\u0000-\u007F]+
#then warn {prefix_error}Unicode characters are prohibited.
#then deny

# Prevent hacked clients spamming messages with UUIDs in them: https://i.imgur.com/zuaROdw.png
#match (\[|)\b[0-9a-f]{8}(\b-|)[0-9a-f]{4}(-|)[0-9a-f]{4}(-|)[0-9a-f]{4}(-\b|)[0-9a-f]{12}(\]|)
#then warn {prefix_error}Please do not type UUIDs into chat.
#then deny

# -----------------------------------------------------------------------------------------------
# Prevent vanish users from chatting in public channels when vanished
# -----------------------------------------------------------------------------------------------

# Makes it so vanished users cannot talk unless they are in the
# admin channel or use "%" at the start of their message in chat
#match ^[^%]
#require script {player_vanished}
#ignore channel admin
#then warn {prefix_error}&7You are in vanished!
#then warn {prefix_error}&7Your message must start with &6%&7 to talk in chat
#then deny
#dont verbose


# This removes the "%" from the start of a vanished
# users message when they talk in chat.
#match ^[%]
#require script {player_vanished}
#ignore channel admin
#strip colors false
#strip accents false
#then replace
#dont verbose

# -----------------------------------------------------------------------------------------------
# Forwarding messages to other channels.
# -----------------------------------------------------------------------------------------------

# Uncomment to forward any message starting with ! to bungee channel
#match ^(\!)(.+)
#then command ch send bungee $2
#strip colors false
#strip accents false
#dont verbose
#then deny

# Uncomment to forward any message starting with . to admin channel
#match ^(\.)(.+)
#then command ch send admin $2
#strip colors false
#strip accents false
#dont verbose
#then deny

# -----------------------------------------------------------------------------------------------
# An example of automatically switching channels depending on player's world when he chats.
# You may also want to go into your localization file and hide these messages by setting 
# Channels.Join sections to '' for keys "Success", "Leave_Reading" to prevent message spam
# when channels are automatically switched.
# -----------------------------------------------------------------------------------------------

# Switches channel for user if not in lobby_chat channel while in the lobby world
#match (.*)
#require world lobby_world
#ignore channel lobby_chat
#then console channel join lobby_chat write {player}|channel sendas {player} lobby_chat $0
#strip colors false
#strip accents false
#then deny

# Switches channel for user if not in survival_chat channel while in a survival world
#match (.*)
#require world survival_world|survival_world_nether|survival_world_the_end
#ignore channel survival_chat
#then console channel join survival_chat write {player}|channel sendas {player} survival_chat $0
#strip colors false
#strip accents false
#then deny

# Switches channel for user if not in creative_chat channel while in a creative world
#match (.*)
#require world creative_world
#ignore channel creative_chat
#then console channel join creative_chat write {player}|channel sendas {player} creative_chat $0
#strip colors false
#strip accents false
#then deny

# -----------------------------------------------------------------------------------------------
# User-friendly hex color placeholders (also works for MC <1.13)
#
# **IMPORTANT** Set 'Rules.Strip_Colors' to 'false' in your settings file first otherwise
# we'll remove colors automatically, unless you add "strip colors false" to each rule.
# -----------------------------------------------------------------------------------------------

#match \%red\%
#then replace #ff0000

#match \%lime\%
#then replace #00ff00

#match \%green\%
#then replace #008000

#match \%blue\%
#then replace #0000ff

#match \%cyan\%
#then replace #00ffff

#match \%magenta\%
#then replace #ff00ff

#match \%yellow\%
#then replace #ffff00

#match \%orange\%
#then replace #ffa500

#match \%gold\%
#then replace #ffd700

#match \%pink\%
#then replace #ffc0cb

#match \%purple\%
#then replace #800080

#match \%brown\%
#then replace #a52a2a

# -----------------------------------------------------------------------------------------------
# Grammar corrections and fun replacements.
# Credit to Tom from Piratecraft (piratemc.com)
# -----------------------------------------------------------------------------------------------

#match \bdis\b
#then replace this

#match \bwanna\b
#then replace want to

#match \bgonna\b
#then replace going to

#match \bu\b
#then replace you

#match \bdia\b
#then replace diamond

#match \bgg\b
#then replace Good Game!

#match \bnp\!\b
#then replace No Problem!

#match \bomg\b
#then replace My golly, this is qwite the shock OwO

#match \bwtf\b
#then replace This princessy-wessy is qwite confuzzled

#match ^(hello|hi|sup)\b
#then replace Greetings from afar fellow pwincesses, this pwincess is looking for some love and compassion

#match ^no
#then replace noh :3

#match ^yes$
#then replace ow yeah :3

#match \bcya\b
#then replace bon voyage, me princess

#match \boml\b
#then replace >.< i iz confuzzled i need some hewp appwreciating dis fwilter

#match \bcannon\b
#then replace shooty-wooty

# -----------------------------------------------------------------------------------------------
# Educate players.
# -----------------------------------------------------------------------------------------------

# Tell people how to do basic things on your server.
#match ^how (do|can) (I|you) (build|claim)(| a) (land|residence)
#then warn {prefix_info}Use &9/help land &7to learn about claiming land!
#then deny

# Prevent spamming "lag".
#match \bl+\s*a+\s*g+
#then warn {prefix_warn}If you believe you have lags, check your &7/ping &6for your internet connection. Anything over 300 won't be helpful! Also check your F3 for FPS client lag. Use our lag guide to diagnose and solve your issue: https://linkToYourGuide.com
#then deny

# -----------------------------------------------------------------------------------------------
# Prevent begging for ranks.
# -----------------------------------------------------------------------------------------------

#match (can|may|would you like if) i (have|be|become|get|has) (op|admin|mod|builder)
#then warn {prefix_info}Currently, we are not looking for new staff.
#then deny

#match (do|are) you (need|wish|looking for) (any|some|one|good) (op|ops|operators|admins|mods|builders|new people|ateam)
#then warn {prefix_info}Currently, we are not looking for new staff.
#then deny

# -----------------------------------------------------------------------------------------------
# Prevent people saying bad things about your server.
# -----------------------------------------------------------------------------------------------

#match this server (is (bad|crappy|shitty)|suck)
#name server hate
#then rewrite I love this server!|I can't behave property due to brain damage!|My bad manners was corrected by server.
#then notify chatcontrol.notify.rulesalert &8[&7ID {rule_name}&8] &7{player}: &f{original_message}
#then console kick {player} &cYour rating will be processed by our staff soon. \nThanks and welcome back!

#match ((admin|op|ateam|server|owner) (is|are)) + *(dick|cock|duck|noob)
#name server hate
#then console kick {player} &cI don't think so.
#then deny

# -----------------------------------------------------------------------------------------------
# Simple chat bots.
# You can then use the {data_name} variable anywhere else, also in PlaceholderAPI!
# -----------------------------------------------------------------------------------------------

# You can create simple helping bots to answer frequently asked questions.
# This will simply listen to the question below and then send player formats/sethome.yml message
#match ^how (do|can|to)(| I| you) (set|create|place)(| a) home
#then warn sethome
#then deny

# Or you can create advanced bots saving and showing data (data is saved permanently)
# See https://github.com/kangarko/ChatControl-Red/wiki/Rules for a tutorial
#match ^(\@bot name)$
#ignore key name
#then warn &8[&dBot&8] &7Please enter your name.
#then deny

#match ^(\@bot name)$
#require key name
#then warn &8[&dBot&8] &7Your name is: {data_name}
#then deny

#match ^(\@bot name) null$
#save key name
#then warn &8[&dBot&8] &7Removed your name.
#then deny

#match ^(\@bot name)(.*)
#save key name "$2".trim()
#then warn &8[&dBot&8] &7Saved your name as: {data_name}.
#then deny

# -----------------------------------------------------------------------------------------------
# Special filter to colorize any commands appearing in the chat. This will turn "Type /help"
# into "Type &c/help" and then intelligently revert back to whatever color the message had
# previously.
#
# PLEASE NOTE that the [#flpc-X] variables are the only two special ones you can use without
# any modifications. If you have Strip_Colors on true in Rules in settings.yml, using any other
# color in the message will get it removed because the match removes colors as per this setting.
# -----------------------------------------------------------------------------------------------

#match (,|\s)\/(\w+)
#then replace [#flpc-i]&c$1/$2[#flpc-1]

# -----------------------------------------------------------------------------------------------
# Smileys (some turds call them emojis)
#
# Notice this may or may not work on your system. Ensure you save the file in UTF-8 encoding,
# and if it still does not work, do not report this, ask your server administrator/hosting.
# Credit to Tom from Piratecraft (piratemc.com)
# -----------------------------------------------------------------------------------------------

#match :-\(
#then replace ☹

#match (:\))|(;\))
#then replace ㋡

#match \:star\:
#then replace ★

#match \:shrug\:
#then replace ¯\\_(ツ)_/¯

#match \:flip\:
#then replace (╯°□°）╯︵ ┻━┻

#match \:wtf\:
#then replace ⚆_⚆

#match \:derp\:
#then replace (◑‿◐)

#match \:love\:
#then replace (✿ ♥‿♥)

#match \:sad\:
#then replace (ಥ﹏ಥ)

#match \:finger\:
#then replace ╭∩╮ ( •_• ) ╭∩╮

#match \:peace\:
#then replace ✌(-‿-)✌

#match \:face\:
#then replace (ᵔᴥᵔ)

#match \:dog\:
#then replace (◕ᴥ◕ʋ)

#match \:bat\:
#then replace /\|\ ^._.^ /\|\

#match \:gun\:
#then replace ︻╦╤─

#match \:butterfly\:
#then replace ƸӜƷ

#match \:tick\:
#then replace &a✓&r

#match \:creep\:
#then replace ԅ(≖‿≖ԅ)

#match \:dead\:
#then replace x⸑x

#match \:fu\:
#then replace ┌П┐(ಠ_ಠ)

#match \:haha\:
#then replace ٩(^‿^)۶

#match \:magicflip\:
#then replace (/¯◡ ‿ ◡)/¯ ~ ┻━┻

#match \:meep\:
#then replace \(°^°)/

#match \:meh\:
#then replace ಠ_ಠ

#match \:no\:
#then replace →_←

#match \:nyan\:
#then replace ~=[,,_,,]:3

#match \:omg\:
#then replace ◕_◕

#match \:rainbowcat\:
#then replace (=^･ｪ･^=))ﾉ彡☆

#match \:cat\:
#then replace ฅ^•ﻌ•^ฅ

#match \:shy\:
#then replace =^_^=

#match \:smirk\:
#then replace ¬‿¬

#match \:unflip\:
#then replace ┬──┬ ノ(ò_óノ)

#match \:up\:
#then replace ↑

#match \:whistle\:
#then replace (っ^з^)♪♬

#match \:wut\:
#then replace ⊙ω⊙

#match \:yay\:
#then replace \( ﾟヮﾟ)/

#match \:rip\:
#then replace rest in spaghetti never forgetti
