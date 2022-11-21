module.exports = {
    run(room) {
        let towers = room.find(FIND_MY_STRUCTURES).filter(structure => structure.structureType == STRUCTURE_TOWER);
        let target = null;


        towers.forEach(tower => {
            let enemies = tower.room.find(FIND_HOSTILE_CREEPS);
            enemies.sort((a, b) => {
                let aHealthPercent = (a.hits / a.hitsMax) * 100;
                let bHealthPercent = (b.hits / b.hitsMax) * 100;

                return aHealthPercent - bHealthPercent;
            });

            if(enemies.length > 0) {
                tower.attack(enemies[0]);
                return;
            }

            if(tower.store.getUsedCapacity(RESOURCE_ENERGY) < 500) {
                return;
            }

            let creeps = tower.room.find(FIND_MY_CREEPS, {
                filter: creep => creep.hits < creep.hitsMax
            });

            if(creeps.length > 0) {
                tower.heal(creeps[0]);
                return;
            }

            if(tower.store.getUsedCapacity(RESOURCE_ENERGY) < 600) {
                return;
            }

            let maintenanceNeeded = tower.room.find(FIND_STRUCTURES, {
                filter: structure => {
                    return structure.hits < structure.hitsMax;
                }
            });

            maintenanceNeeded.sort((a, b) => a.hits < b.hits);
            maintenanceNeeded.sort((a, b) => ((a.hits / a.hitsMax) * 100) < ((b.hits / b.hitsMax) * 100));

            if(maintenanceNeeded.length > 0) {
                tower.repair(maintenanceNeeded[0]);
                return;
            }
        });
    }
}
