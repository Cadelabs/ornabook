<div class="title">Kingdoms</div>

# Overview
## How to find
## Advantages
# Resources
# Gauntlets
Kingdom Gauntlets are one of the few ways of obtaining Kingdom Orns and the most lucrative if carried out seriously.
A Gauntlet has a certain number of floors, where a member is randomly selected to fight a monster or a boss.
Once sufficient fights are won, the Gauntlet is won and the Kingdom receives Kingdom Orns.

### Basic overview
Kingdom Gauntlets may be started by Officers in exchange for Kingdom Gold.
The game randomly selects, for each floor, one unbenched Kingdom Member to fight a random enemy.
Members selected to fight in the Gauntlet receive a Push Notification and can take their fight regardless of whether fights from the previous floors have been taken.
Once sufficient fights have been won, the Kingdom is victorious in its Gauntlet and receives Kingdom Orns.
Officers can then start another Kingdom Gauntlet.

### Starting a Gauntlet
Officers can start a Gauntlet only if there is no Gauntlet already started and if there are at least 6 unbenched Members.
At least 1 hour must have elapsed between 2 starts of a Gauntlet (this timer starts when _starting_ a Gauntlet; there needs not be 1 hour between the end of a Gauntlet and the start of the next one).
The cost of a Kingdom Gauntlet is `2400 * roster_size` Kingdom Gold.
Benched Members contribute to the roster size (benching Members does not reduce the cost of a Gauntlet).

The number of floors in a Gauntlet depends on the number of unbenched members in the Kingdom at the time the Gauntlet is started.
The rules are as follows:
 * If there are 6 to 14 unbenched Members, the Gauntlet has 10 floors.
 * If there are 15 to 19 unbenched Members, the Gauntlet has 12 floors.
 * From 20 unbenched Members onwards, the number of floors increases by 4 for every 5 members.

This gives the following table (ranges inclusive):

|Unbenched Members|Number of Floors|
|:---------------:|:--------------:|
|       6-14      |       10       |
|      15-19      |       12       |
|      20-24      |       16       |
|      25-29      |       20       |
|      30-34      |       24       |
|      35-39      |       28       |
|      40-44      |       32       |
|      45-49      |       36       |
|        50       |       40       |

For each floor, a random unbenched Member is selected.
If a Member is selected for a floor, they are not excluded from the selection pool for later floors.
This means that a Member may be assigned multiple floors.

Enemies are chosen randomly, from the list of monsters and bosses of the tier of the Member (a T5 Member will face a T5 enemy).
That list includes enemies from active events, but not from inactive events.
They have a rare chance of being Berserk.

All characteristics of a Gauntlet are set when it is started.
Benching or unbenching Members while the Gauntlet is being fought does not change the number of floors or the number of allowed losses.

```admonish todo "TODO(16/06/2024, ethiraric)"
What's the rate of a zerk mob?
```

### Fighting a Gauntlet
Gauntlets can be fought through the Gauntlet tab in the Kingdom menu.
Fights use the player's Dungeon loadout.
A fight is considered lost if the player is defeated.
However, before being defeated, they can flee from the fight freely and as many times as they want.
When the fight is restarted, the enemy will be back to full health.
Even though fights take place in a dungeon, they award full Gold/Orn/Exp.

Should a fight prove too hard for a Member or if a Member is inactive, Officers can "shuffle" a Gauntlet floor.
Shuffling replaces the fight by a new one, following the same rules described in [Starting a Gauntlet](#starting-a-gauntlet).
The replaced floor is removed, all further floors are moved up by one and the new fight is appended at the bottom of the Gauntlet.
Shuffling is instantaneous and on a 20mn timer, shared by all Officers.
Shuffling can be done on any unfought floor, except those that were started (i.e.: a Member is currently fighting it, or has attempted it and fled).

Shuffling often and cleverly is the key to completing Gauntlets fast.
A lot of organization and knowledge of Members' habits are required in order to efficiently shuffle.
Ideally:
  * Try to shuffle as often as possible.
  * Shuffle Members who may lose their fight or may not take it promptly (e.g.: it's nighttime in their timezone, they went hiking with poor reception, ...).
  * Avoid shuffling Members who have multiple floors to fight.

### End of a Gauntlet
A Gauntlet ends when it is either won or lost.

Kingdoms are allowed a few lost fights in the Gauntlet.
If, when the Gauntlet was started, there were less than 15 unbenched Members, the Kingdom is allowed 2 lost fights.
Otherwise, if there were 15 or more unbenched Members, the Kingdom is allowed 3 lost fights.
Upon losing the 2nd or 3rd fight respectively, the Gauntlet is lost and the Kingdom gains no Kingdom Orns.
A new Gauntlet can be started by Officers, if the 1h cooldown has elapsed.

In order for the Gauntlet to be won, there has to be at least `nb_floors - (losses_allowed + 1) + losses` floors won.
This means that not everybody has to take their fight before a new Gauntlet can be started.
Most typically, for 3 losses allowed and no lost fight, all floors but 4 have to be won.
In this example, the 5th-to-last won fight will complete the Gauntlet, provide the Kingdom with Orns, and allow the Officers to start the next Gauntlet (if the 1h cooldown has elapsed).

### Rewards
The number of Orns gained depends on the number of floors and their tiers.
All floors in the Gauntlet at the time the Gauntlet is won, whether fought or not, will reward with Kingdom Orns.

The amount of Kingdom Orns earned for each floor depending on the tier is summarized below:

|Tier|Kingdom Orns|
|:--:|:----------:|
|  2 | 62 |
|  3 | 125 |
|  4 | 200 |
|  5 | 300 |
|  6 | 425 |
|  7 | 550 |
|  8 | 900 |
|  9 | 1000 |
| 10 | 1250 |
| 11 | 1250 |

The amount of Kingdom Orns earned for a Gauntlet is the sum of the Kingdom Orns earned for each floor (whether won, lost or unfought) of the Gauntlet at the moment of completion.

# Wars
