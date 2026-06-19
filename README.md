## TODO
* make dodge so that lerping is purely a slow return of gravity but player can still move left and right
  - basically lerping is no longer a lone state... idk how you're gonna handle this with
    the current statemachine setup.
* make it so that some statemachines are added only once they receive the weapon...
  - like the combat state machine should only be added once received steelsword or zombie_arm
  - some other state would be for like potions, planting, or some other item held by player
* fix up again the new dodge_stamina, user can hold dodge, (maybe a feature??)
* make dodge regenerates for each successful dodge (DONE)
  - BUT the health.immune no longer is used for ignoring full collision check
    since the player needs to get hit first to gain successful dodge
* add stamina for dodging and skill uses
  - NEED TO TEST WHETHER immunity in lerping is viable or too easy
* use the statemachine components for the weapon states
* add player attack combos (different animation for some key combos)
  - if player attacks while dodging, play a different animation
  - if player uses some key: he can use the skill of the weapon
    - skill decreases stamina
    - if skill is used

  - hold dodge before attacking to do dodge attack (allows dodge attack)
    - cannot dodge while attaacking
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
