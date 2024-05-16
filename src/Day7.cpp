#include <iostream>
#include <fstream>
#include <ranges>
#include <string>
#include <vector>
#include <map>
#include <algorithm>

using namespace std;

const char JOKER = 'J';

struct Hand {
    int bid, type;
    string cards;
};

enum HAND_TYPE {
    HIGH_CARD = 1,
    ONE_PAIR,
    TWO_PAIR,
    THREE_KIND,
    FULL_HOUSE,
    FOUR_KIND,
    FIVE_KIND
};

int categorise_hand(const string cards, const string card_strengths,
                    bool jokers);

int find_best_alternate(string cards, const string &strengths,
                        const int jokers_count);

vector<string> splitString(const string &str);

int cardValueToNumber(char cardValue);

bool compareHands(const string &hand1, const string &hand2);

bool isFiveOfAKind(const string &str);

bool isFourOfAKind(const string &str);

bool isFullHouse(const string &str);

bool isThreeOfAKind(const string &str);

bool hasTwoPairs(const string &str);

bool hasOnePair(const string &str);

int part_one();

int part_two();

int main() {

    cout << part_one() << endl;
    cout << part_two() << endl;

    return 0;
}

int part_two() {
    vector<Hand> hands;
    int result = 0;
    string card_strengths = "J23456789TQKA";
    fstream newfile;
    newfile.open("../day_7_input.txt", ios::in);
    if (newfile.is_open()) {
        string tp;
        while (getline(newfile, tp)) {
            string cards;
            [[maybe_unused]] int bid;
            cards = splitString(tp)[0];
            stoi(splitString(tp)[1]);
            hands.push_back(Hand{bid, categorise_hand(cards, card_strengths, true), cards});
        }
        sort(hands.begin(), hands.end(), [card_strengths](Hand a, Hand b) {
            if (a.type != b.type) {
                return a.type < b.type;
            }
            for (size_t i = 0; i < a.cards.length(); i++) {
                int score_a = card_strengths.find(a.cards[i]);
                int score_b = card_strengths.find(b.cards[i]);

                if (score_a != score_b) {
                    return score_a < score_b;
                }
            }
            return true;
        });
        for (size_t rank = 0; rank < hands.size(); rank++) {
            result += hands[rank].bid * (rank + 1);
        }

        cout << result << endl;
    }
}

int part_one() {

    vector<string> high_card;
    vector<string> one_pair;
    vector<string> two_pair;
    vector<string> three_of_a_kind;
    vector<string> full_house;
    vector<string> four_of_a_kind;
    vector<string> five_of_a_kind;

    fstream newfile;
    newfile.open("../day_7_input.txt", ios::in);
    if (newfile.is_open()) {
        string tp;
        while (getline(newfile, tp)) {
            string hand;
            hand = splitString(tp)[0];
            if (isFiveOfAKind(hand)) {
                five_of_a_kind.push_back(tp);
                continue;
            }
            if (isFourOfAKind(hand)) {
                four_of_a_kind.push_back(tp);
                continue;
            }
            if (isFullHouse(hand)) {
                full_house.push_back(tp);
                continue;
            }
            if (isThreeOfAKind(hand)) {
                three_of_a_kind.push_back(tp);
                continue;
            }
            if (hasTwoPairs(hand)) {
                two_pair.push_back(tp);
                continue;
            }
            if (hasOnePair(hand)) {
                one_pair.push_back(tp);
                continue;
            }
            high_card.push_back(tp);
        }

        sort(high_card.begin(), high_card.end(), compareHands);
        sort(one_pair.begin(), one_pair.end(), compareHands);
        sort(two_pair.begin(), two_pair.end(), compareHands);
        sort(three_of_a_kind.begin(), three_of_a_kind.end(), compareHands);
        sort(full_house.begin(), full_house.end(), compareHands);
        sort(four_of_a_kind.begin(), four_of_a_kind.end(), compareHands);
        sort(five_of_a_kind.begin(), five_of_a_kind.end(), compareHands);

        int result = 0;
        int counter = 0;
        for (auto bid: ranges::reverse_view(high_card)) {
            ++counter;
            int real_bid = stoi(splitString(bid)[1]);
            result += real_bid * counter;
        }
        for (auto bid: ranges::reverse_view(one_pair)) {
            ++counter;
            int real_bid = stoi(splitString(bid)[1]);
            result += real_bid * counter;
        }
        for (auto bid: ranges::reverse_view(two_pair)) {
            ++counter;
            int real_bid = stoi(splitString(bid)[1]);
            result += real_bid * counter;
        }
        for (auto bid: ranges::reverse_view(three_of_a_kind)) {
            ++counter;
            int real_bid = stoi(splitString(bid)[1]);
            result += real_bid * counter;
        }
        for (auto bid: ranges::reverse_view(full_house)) {
            ++counter;
            int real_bid = stoi(splitString(bid)[1]);
            result += real_bid * counter;
        }
        for (auto bid: ranges::reverse_view(four_of_a_kind)) {
            ++counter;
            int real_bid = stoi(splitString(bid)[1]);
            result += real_bid * counter;
        }
        for (auto bid: ranges::reverse_view(five_of_a_kind)) {
            ++counter;
            int real_bid = stoi(splitString(bid)[1]);
            result += real_bid * counter;
        }
        return result;

    }
}

bool isFiveOfAKind(const string &str) {
    map<char, int> counts;
    for (char ch: str) {
        counts[ch]++;
        if (counts[ch] == 5) {
            return true;
        }
    }
    return false;
}

bool isFourOfAKind(const string &str) {
    map<char, int> counts;
    for (char ch: str) {
        counts[ch]++;
        if (counts[ch] == 4) {
            return true;
        }
    }
    return false;
}

bool isFullHouse(const string &str) {
    map<char, int> counts;
    for (char ch: str) {
        counts[ch]++;
    }
    bool foundThree = false;
    bool foundTwo = false;
    for (const auto &pair: counts) {
        if (pair.second == 3) {
            foundThree = true;
        } else if (pair.second == 2) {
            foundTwo = true;
        }
    }
    return foundThree && foundTwo;
}

bool isThreeOfAKind(const string &str) {
    map<char, int> counts;
    for (char ch: str) {
        counts[ch]++;
        if (counts[ch] == 3) {
            return true;
        }
    }
    return false;
}

bool hasTwoPairs(const string &str) {
    map<char, int> counts;
    for (char ch: str) {
        counts[ch]++;
    }
    bool foundfpair = false;
    bool foundspair = false;
    for (const auto &pair: counts) {
        if (pair.second == 2) {
            if (!foundfpair) {
                foundfpair = true;
            } else {
                foundspair = true;
            }
        }
    }
    return foundfpair && foundspair;
}

bool hasOnePair(const string &str) {
    map<char, int> counts;
    for (char ch: str) {
        counts[ch]++;
        if (counts[ch] == 2) {
            return true;
        }
    }
    return false;
}

vector<string> splitString(const string &str) {
    size_t pos = str.find(' ');
    string fString, sString;
    vector<string> result;

    if (pos != string::npos) {
        fString = str.substr(0, pos);
        sString = str.substr(pos + 1);
        result.push_back(fString);
        result.push_back(sString);
    } else {
        result.push_back(str);
        result.push_back("");
    }

    return result;
}

int cardValueToNumber(char cardValue) {
    if (isdigit(cardValue)) {
        return cardValue - '0';
    } else {
        switch (cardValue) {
            case 'T':
                return 10;
            case 'J':
                return 11;
            case 'Q':
                return 12;
            case 'K':
                return 13;
            case 'A':
                return 14;
            default:
                return 0;
        }
    }
}

bool compareHands(const string &hand1, const string &hand2) {
    for (size_t i = 0; i < hand1.size(); ++i) {
        int value1 = cardValueToNumber(hand1[i]);
        int value2 = cardValueToNumber(hand2[i]);
        if (value1 != value2) {
            return value1 > value2;
        }
    }
    return false;
}


int find_best_alternate(string cards, const string &strengths,
                        const int jokers_count) {
    switch (categorise_hand(cards, strengths, false)) {
        case FIVE_KIND:
            return FIVE_KIND;
        case FOUR_KIND:
            return FIVE_KIND;
        case FULL_HOUSE:
            switch (jokers_count) {
                case 1:
                    return FOUR_KIND;
                case 2:
                    return FIVE_KIND;
                case 3:
                    return FIVE_KIND;
            }
        case THREE_KIND:
            return FOUR_KIND;
        case TWO_PAIR:
            switch (jokers_count) {
                case 1:
                    return FULL_HOUSE;
                case 2:
                    return FOUR_KIND;
            }
        case ONE_PAIR:
            return THREE_KIND;
        case HIGH_CARD:
            return ONE_PAIR;
        default:
            exit(EXIT_FAILURE);
    }
}

static map<char, int> count_cards(string cards) {
    map<char, int> cards_count;

    for (const char &c: cards) {
        cards_count[c]++;
    }

    return cards_count;
}

int categorise_hand(const string cards, const string card_strengths,
                    bool jokers) {
    map<char, int> cards_count = count_cards(cards);

    if (jokers && cards_count.count(JOKER)) {
        return find_best_alternate(cards, card_strengths, cards_count.at(JOKER));
    }

    switch (cards_count.size()) {
        case 5:
            return HIGH_CARD;
        case 4:
            return ONE_PAIR;
        case 3:
            return any_of(cards_count.begin(), cards_count.end(),
                               [](auto count) { return count.second == 2; })
                   ? TWO_PAIR
                   : THREE_KIND;
        case 2:
            return abs(cards_count.begin()->second -
                            cards_count.rbegin()->second) == 3
                   ? FOUR_KIND
                   : FULL_HOUSE;
        case 1:
            return FIVE_KIND;
        default:
            exit(EXIT_FAILURE);
    }
}

