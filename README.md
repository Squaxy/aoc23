# aoc23
advent of code 2023

## Task 1
    - why isnt the part-2 solution working, every manual test and edgecase i can think of works during test
    - the value also seems plausible
## Task 2
    - no problems at all
## Task 3
    - verstehe schon wieder nicht was geht, kann leider nicht mehr debuggen
    - chatgpt hat meine methode in part 1 nur rusty'er gemacht und plötzlich geht es, logikfehler war es also nicht.
    - part2 nicht schön aber geht so, ich habe kein bock den compiler zu bekämpfen, darum die get_clone(). Vermutlich hätte man games mehr mit referenzen handlen sollen
## Task 4
    - part 1 ziemlich leicht
    - part 2 klingt ziemlich nach einem rekursions problem
    - puh habs geschafft. mit diesem get_clone() schonwieder, ich kam nicht drum herum diese wieder zu implementieren, aber richtig fühlt sich das nicht an
## Task 5
    - war eigentlich ganz ok
    - was geht ab, ram overflow
      - aber es kann eigentlich nicht an der recursion liegen, die wird ja für jeden seed reset
      - ist die seed to location map einfach zu groß?
      - bessere Lösung wäre: seed ranges komplett zu propagieren. Wenn eine seed range nicht komplett in eine
      gardenersentry range reinpasst, kann man die seed range in mehrere seed ranges splitten. hier auch wieder rekursiv? gib der aufgabe nochmal einen versuch!
## Task 6
    - ok das war viel zu einfach
    - auch part 2
    - map reduce zu satisfying, bin schon stolz das ich die zeile code mal eben in unter einer minute hingeknallt habe
## Task 7
    - something feels off about part 1, the way of the solution is nice, but
    - the mutability and reference passing in the code feels weird
    - part2 was interesting, realizing that some possibilities are not feasible, the flow of checking the highest value first and then breaking
      - made it easier
## Task 8
    - wollte zuerst ein OOP ansatz wählen, aber ist doch völliger quatsch, wenns auch mal schnell geht
    - part 2 giftig, ob ma wohl mit selber iterativer lösung zum ziel kommt. vermutlich läuft das programm ewig
      - die lösung ist definitiv mit "kleinstem gemeinsamen teiler" zu lösen, aber hab kein bock mehr