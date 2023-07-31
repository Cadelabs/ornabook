# Status Effects
Status effects are changes that affect your character during combat. They can increase or decrease your stats, inflict damage over time, cause death, ... or multiple of those at the same time. One can get a status effect through:

  - One of their own spells
  - A spell of an enemy
  - A spell of a follower
  - A spell of an ally, while playing in party mode
  - An equipped item

Some status effects are temporary, while some other are permanent. Some effects do have both a temporary and a permanent variation. Temporary status effects have a chance of fading each turn and are usually prefixed by `T.`.

## Stats altering status effects
### Generic status effects
These status effects change how the base stats (attack, magic, defense, resistance, dexterity) behave:
    - :statuses/def: Defense: Change to the base defense stat
    - :statuses/res: Resistance: Change to the base resistance stat
    - :statuses/atk: Attack: Change to the damage a physical skill/spell does (doesn't change the base attack value)
    - :statuses/mag: Magic: Change to the damage a magical skill/spell does (doesn't change the base magic value)
    - :statuses/dex: Dexterity: Change to the base dexterity stat
    - :statuses/crit: Critical Chance: Change to the chance of dealing a critical strike
    - :statuses/windswept: Windswept: Decrease to the chance of dealing a critical strike
    - :statuses/all: All: Change to :statuses/def: :statuses/res: :statuses/atk: :statuses/mag: all at once

```admonish todo "TODO(08/06/2023, ethiraric)"
Does :statuses/all: change dexterity?
```

There exists different variations of the above effects, often referred to as "up"/"down", "single"/"double"/"triple" and "temporary"/"permanent". Temporary effects (whether positive or negative) are denoted with purple arrows, permanent positive effects are denoted with green arrows and permanent negative effects are denoted with orange arrows.

The table below describes, for each buff, the alteration percentage for each combination of stat and arrows. Empty cells denote combinations that do not exist and triple question marks (`???`) combinations that do exist but whose alteration is not known or hasn't yet been contributed.

| Stat(s) altered | :statuses/td_tmp: | :statuses/dd_tmp: | :statuses/sd_tmp: | :statuses/su_tmp: | :statuses/du_tmp: | :statuses/tu_tmp: | :statuses/sd: | :statuses/dd: | :statuses/su: | :statuses/du: |
|:---------------:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|:-----------------:|:-------------:|:-------------:|:-------------:|:-------------:|
| :statuses/def::statuses/res::statuses/atk::statuses/mag: | -100% | -80% | -20% | +25% | +50% | +100% | -20% | -80% | +25% | +50% |
| :statuses/dex: |     |     | ??? | ??? |     | ??? |     |     | ??? |     |
| :statuses/all: |     |     |     | +20% |     | +100% |     |     |     |     |
| :statuses/crit: |     |     |     | +20% | ??? | +80%* |     |     | +20% |     |

*:statuses/crit_tu_tmp: also grants +10%:statuses/atk: (no additional effect on :statuses/mag:)

```admonish note
Individual buff combinations are considered different buffs and stack *multiplicatively*. This means that :statuses/def_su:+:statuses/def_du: grants a `1.25*1.5=1.875` factor to defense (+87.5%). However, using different sources for the same buff does not apply the buff twice. For example, if :statuses/mag_su: is granted using either of Magic Boost or Wyvern Speed, casting the other spell will not grant another :statuses/mag_su:.
```

### Gaits
Gaits are a family of buffs that can only be applied to oneself. They are learnt by the Beowulf/Bestla at levels 225 and 230. While Gaits are permanent buffs, only one Gait can be active at a time. Casting a Gait spell while another Gait is active will replace the active Gait, as opposed to granting a new status effects.

Gaits allow their caster to boost one of their stats at the expense of 2 others. The boosted stat is increased by 50%, while the 2 sacrificed stats are decreased by 90% each.

Below are the 4 Gaits and their effects:

| Gait | +++ | --- | --- |
|:----:|:---:|:---:|:---:|
| :statuses/eir: Eir | +50% :statuses/res: | -90% :statuses/atk: | -90% :statuses/def: |
| :statuses/gunnr: Gunnr | +50% :statuses/atk: | -90% :statuses/mag: | -90% :statuses/res: |
| :statuses/kara: Kára | +50% :statuses/def: | -90% :statuses/mag: | -90% :statuses/res: |
| :statuses/snotra: Snotra | +50% :statuses/mag: | -90% :statuses/atk: | -90% :statuses/def: |

### Miscellaneous effects
| Effect | Description |
|:------:|:-----------:|
| :statuses/berserk_su: Dmg ↑ | Self buff from Berserk. Increases damage output by 50% at the cost of 5% of max HP per turn. |
| :statuses/berserk_du: All ↑ | Self buff from Berserk II. Boosts :statuses/atk::statuses/mag::statuses/def::statuses/res: by 25% each at the cost of 10% of max HP per turn. |
| :statuses/berserk_tu: All ↑↑ | Self buff from Berserk III. Boosts :statuses/atk::statuses/mag::statuses/def::statuses/res: by 50% each at the cost of 25% of max HP per turn. |
| :statuses/break: T. All ↓ | Debuff inflicted by Break. |

```admonish todo "TODO(31/07/2023, ethiraric)"
What does "Break" do exactly?
```

### Monster only status effects
The Cerus family of raid bosses have two spells named "Defend" that apply a self-buff. One of these applies a :statuses/def:Defending status buff, while the other applies a :statuses/res:Defending buff. These buffs increase Cerus' defense or resistance respectively and are permanent. However, much like Gaits, the two buffs cannot be active at the same time and applying one will remove the other.

## Status Afflictions
```admonish todo
```

### Controls and DoTs
```admonish todo
```

### Sigils
```admonish todo
```

### Blights
```admonish todo
```

## Elemental resistances and alignments
```admonish todo
```

## Calls
```admonish todo
```

