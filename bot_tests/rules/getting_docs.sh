#! /bin/bash


# part 1: reading website into rules.txt
# use IPv4
echo "downloading warlight 2 rules..."
sleep 2 # slow down if your internet is fast
wget -4 http://theaigames.com/competitions/warlight-ai-challenge-2/rules -O temp_rules.html
lynx -nolist -dump temp_rules.html > rules_2.txt
rm temp_rules.html


# part 2: reading website into gettingstarted.txt

echo "downloading warlight 2 \"getting started\"..."
sleep 1
wget -4 -O temp_gettingstarted.html http://theaigames.com/competitions/warlight-ai-challenge-2/getting-started
lynx -nolist -dump temp_gettingstarted.html > gettingstarted_2.txt
rm temp_gettingstarted.html



echo "downloading warlight 1 rules..."
sleep 2
wget -4 http://theaigames.com/competitions/warlight-ai-challenge/rules -O temp_rules.html
lynx -nolist -dump temp_rules.html > rules_1.txt
rm temp_rules.html

echo "downloading warlight 1 \"getting started\"..."
sleep 1
wget -4 -O temp_gettingstarted.html http://theaigames.com/competitions/warlight-ai-challenge/getting-started
lynx -nolist -dump temp_gettingstarted.html > gettingstarted_1.txt
rm temp_gettingstarted.html

echo ""
echo ...done
