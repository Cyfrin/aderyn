cd aderyn_pilot
touch archive.zip
rm archive.zip
zip -r9 archive.zip bot_starter_pack -x "bot_starter_pack/target/*" -x "bot_starter_pack/.git/*"