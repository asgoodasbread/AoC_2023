/*
## Medival Style Software License Version 1 - MSSLv1
**THE GRAND PACT OF DIGITAL GOODS LIBERATION**

In the Year of our Lord two thousand and twenty four, in the reign of the most high and mighty
kihau, the great programmer, a humble vassal of the realm of digital goods,
doth grant unto thee, a noble knight of the digital realm, a pact of most singular and extraordinary nature, to wit:

**ARTICLE THE FIRST: OF THE RIGHT TO UTILIZE**

Thou shalt have the right, privilege, and liberty to employ the digital goods, known henceforth as
"Timer in C", for thy own purposes, whether for the greater good or for the pursuit of thy own
interests, provided always that thou dost not utilize the digital goods in a manner contrary to the laws of the land,
the dictates of chivalry, or the principles of honour.

**ARTICLE THE SECOND: OF THE RIGHT TO MODIFY**

Thou shalt have the right, privilege, and liberty to alter, amend, and modify the digital goods, to add thy own code,
and to make them thine own, provided always that thou dost preserve the integrity of the original work, and dost not
delete, efface, or obscure the notices of the original authors, who shall be held in esteem and reverence as the
progenitors of the digital goods.

**ARTICLE THE THIRD: OF THE RIGHT TO DISTRIBUTE**

Thou shalt have the right, privilege, and liberty to disseminate, propagate, and distribute the digital goods, whether
in their original form or modified by thy own hand, provided always that thou dost distribute the digital goods under
the terms of this pact, and not under any other pact or terms, lest thou fall prey to the slings and arrows of
outrageous fortune.

**ARTICLE THE FOURTH: OF THE OBLIGATION TO SHARE**

If thou dost modify the digital goods, thou shalt be bound by an oath of fealty to share thy modifications with the
world, that others may benefit from thy labours, and that the digital goods may flourish like a garden in springtime.
Thou shalt make available thy modified version of the digital goods, under the terms of this pact, for all to use and
modify, lest thou be deemed a hoarder of knowledge and a miser of innovation.

**ARTICLE THE FIFTH: OF THE PROHIBITION ON RESTRICTION**

Thou shalt not impose any restrictions on the use of the digital goods, nor shalt thou exact any tribute, fee, or
pecuniary consideration from those who would use the digital goods, lest thou be deemed a tyrant and a oppressor of
the digital realm.

**ARTICLE THE SIXTH: OF THE WARRANTY OF MERCHANTABILITY**

The digital goods are provided "as is", without warranty of any kind, express or implied, save for the warranty of
fitness for the purposes of idle amusement and frivolous diversion. The might kihau doth not
warrant that the digital goods will meet thy requirements, nor that they will be free from errors, bugs, or other
defects, which may cause thee to rend thy garments and pull thy hair in frustration.

**ARTICLE THE SEVENTH: OF THE LIMITATION OF LIABILITY**

In no event shall the might kihau be liable for any damages, whether direct, indirect,
incidental, or consequential, arising from the use of the digital goods, save for the liability of being deemed a
party to a most grievous and heinous breach of the peace.

**ARTICLE THE EIGHTH: OF THE GOVERNING LAW**

This pact shall be governed by the laws of the land, and any disputes arising from its interpretation or enforcement
shall be resolved through the courts of the realm, or by trial by combat, at the option of the parties.

**ARTICLE THE NINTH: OF THE ENTIRE AGREEMENT**

This pact constitutes the entire agreement between thee and the might kihau regarding the
digital goods, and supersedes all prior or contemporaneous agreements or understandings, whether written or oral,
save for the unwritten code of chivalry and the unspoken understanding of honour among gentlemen.

By using, modifying, or distributing the digital goods, thou dost acknowledge that thou hast read, understood, and
agreed to the terms of this pact, and dost bind thyself to its provisions, under pain of perjury and the wrath of the
digital gods.

May Fortune smile upon thee, noble knight of the digital realm, and may thy code be ever bug-free and thy digital
goods ever prosperous!
*/

#ifndef TIMER_H
#define TIMER_H
#include <stdio.h>

#if defined(__linux)
typedef struct {
        struct timespec time;
    } Timer;
#elif defined(_WIN32)
#include <windows.h>
typedef struct {
    LARGE_INTEGER time;
} Timer;
#else // Other platforms, lower precision
typedef struct {
        clock_t time;
    } Timer;
#endif

void timer_init(void);
Timer timer_new(void);
void timer_start(Timer *timer);
void timer_stop(Timer *timer);
unsigned long timer_elapsed(const Timer timer);

void timer_print(const char* prompt, Timer timer);

#endif // TIMER_H