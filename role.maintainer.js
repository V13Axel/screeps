var roleMaintainer = {
    desiredNumber: 2,
    definition: [WORK, CARRY, MOVE],
    partsBudgets: {
        [WORK]: {
            costModifier: .3,
            cost: 100
        },
        [CARRY]: {
            costModifier: .40,
            cost: 50
        },
        [MOVE]: {
            costModifier: .30,
            cost: 50
        },
    },

    /** @param {Creep} creep **/
    run: function(creep) {
        switch(creep.memory.action) {
            case 'fixing':
                this.fix(creep);
                break;
            case 'harvest':
                this.harvest(creep);
                break;
            case 'waiting':
                this.wait(creep);
                break;
            default:
                creep.memory.action = 'harvest';
        }

    },


    harvest: function(creep) {
        if(creep.store.getFreeCapacity() == 0) {
            creep.memory.action = 'waiting';
            return;
        }

        if(!creep.memory.source) {
            let sources = creep.room.find(FIND_SOURCES);

            creep.memory.source = sources[Math.floor(Math.random()*sources.length)];
        }


        if(creep.harvest(creep.memory.source) == ERR_NOT_IN_RANGE) {
            creep.moveTo(creep.memory.source, {visualizePathStyle: {stroke: '#ffaa00'}});
        }
    },

    wait: function(creep) {
        let spawns = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return structure.structureType == STRUCTURE_SPAWN
            }
        });
        creep.moveTo(spawns[0]);
    },

    fix: function(creep) {
        let attempt = creep.repair(creep.memory.fixing);

        switch (attempt) {
            case ERR_NOT_IN_RANGE:
                creep.moveTo(creep.memory.fixing);
                break;
            default:
                console.log(creep.name + "Repair attmept returned code: " + attempt);
                break;
        }

        let targets = creep.room.find(FIND_STRUCTURES, { filter: creep.memory.fixing});
        
        if(targets.length != 1){
            creep.memory.fixing = null;
        }

        var needingRepair = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return (structure.structureType == STRUCTURE_ROAD ||
                        structure.structureType == STRUCTURE_WALL ||
                        structure.structureType == STRUCTURE_RAMPART) && 
                        (structure.hits < structure.hitsMax * 0.7 && 
                        structure.hitsMax - structure.hits < creep.store.getUsedCapacity()
                        );
            }
        });

        if(needingRepair.length < 1) {
            creep.memory.action = 'harvest';
            return;
        }

        creep.memory.action = 'fixing';
        creep.memory.fixing = needingRepair[0];

        return;
    }
};

module.exports = roleMaintainer;
