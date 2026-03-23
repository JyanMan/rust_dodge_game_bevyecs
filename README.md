## TODO
* use the statemachine components for the weapon states
* use statemachine for the zombie states
* add stamina for dodging and skill uses
  - NEED TO TEST WHETHER immunity in lerping is viable or too easy
* add player attack combos (different animation for some key combos)
  - if player attacks while dodging, play a different animation
  - if player uses some key: he can use the skill of the weapon
    - skill decreases stamina
    - if skill is used

  - hold dodge before attacking to do dodge attack (allows dodge attack)
    - cannot dodge while attacking
  - slam attack
* now make the weapon have a chance to drop
* add zombie hit animation
* fix frame flutter bug
* optimize entity_quad fetching for entities overlapping

## MECHANIC
* if a dodge is successful and was attacked during the act...
  the player completely refills his dodge stamina.
  This allows it so that unlimited dodges can be achieved
  even at the beginning of the game, as long as the player was
  attacked while doing so.
