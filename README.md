## TODO
* now make the weapon have a chance to drop
* add zombie hit animation
* add hit particle like blood or something
* add player attack combos (different animation for some key combos)
  - hold dodge before attacking to do dodge attack (allows dodge attack)
    - cannot dodge while attacking
  - slam attack
* fix frame flutter bug
* optimize entity_quad fetching for entities overlapping

## MECHANIC
* if a dodge is successful and was attacked during the act...
  the player completely refills his dodge stamina.
  This allows it so that unlimited dodges can be achieved
  even at the beginning of the game, as long as the player was
  attacked while doing so.
