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

            if(tower.store.getUsedCapacity() < 1050) {
                return;
            }

            let creeps = tower.room.find(FIND_MY_CREEPS).filter(creep => creep.hits < creep.hitsMax);
            if(creeps.length > 0) {
                tower.heal(creeps[0]);
                return;
            }

            if(tower.store.getUsedCapacity() < 1550) {
                return;
            }

            let maintenanceNeeded = tower.room.find(FIND_STRUCTURES).filter(structure => {
                return structure.hits < structure.hitsMax;
            });

            if(maintenanceNeeded.length > 0) {
                tower.repair(maintenanceNeeded[0]);
                return;
            }
        });
    }
}
