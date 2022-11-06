var roleMaintainer = {
    desiredNumber: 1,
    definition: [WORK, CARRY, MOVE],
    partsBudgets: {
        [WORK]: {
            costModifier: .5,
            cost: 100
        },
        [CARRY]: {
            costModifier: .25,
            cost: 50
        },
        [MOVE]: {
            costModifier: .25,
            cost: 50
        },
    },

    /** @param {Creep} creep **/
    run: function(creep) {
        creep.say(creep.memory.action);

        switch(creep.memory.action) {
            case 'cleanup':
                this.cleanup(creep);
                break;
            case 'fixing':
                this.fix(creep);
                break;
            case 'harvest':
                this.harvest(creep);
                break;
            case 'choosing':
                this.choose(creep);
                break;
            default:
                creep.memory.action = 'cleanup';
        }

    },

    cleanup: function(creep) {
        let dropped = creep.room.find(FIND_DROPPED_RESOURCES);
        if(!dropped.length) {
            creep.memory.action = 'choosing';
            return;
        }

        if(creep.pickup(dropped[0]) == ERR_NOT_IN_RANGE) {
            creep.moveTo(dropped[0]);
        }
    },

    harvest: function(creep) {
        if(creep.store.getFreeCapacity() == 0) {
            creep.memory.action = 'choosing';
            return;
        }

        if(!creep.memory.source) {
            let sources = creep.room.find(FIND_SOURCES);

            creep.memory.source = sources[Math.floor(Math.random()*sources.length)].id;

            console.log(creep.memory.source);
        }


        let target = Game.getObjectById(creep.memory.source);
        let attempt = creep.harvest(target); 
        switch (attempt) {
            case ERR_NOT_IN_RANGE:
                creep.moveTo(target, {visualizePathStyle: {stroke: '#ffaa00'}});
                break;
            case ERR_INVALID_TARGET:
                console.log("Invalid target", attempt, JSON.stringify(target));
                creep.memory.source = null;
                break;
            default:
                console.log(attempt);
                
        }
    },

    choose: function(creep) {
        let spawns = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return structure.structureType == STRUCTURE_SPAWN
            }
        });
        creep.moveTo(spawns[0]);

        var needingRepair = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return (structure.structureType == STRUCTURE_ROAD ||
                        structure.structureType == STRUCTURE_WALL ||
                        structure.structureType == STRUCTURE_RAMPART) && 
                        (structure.hits < structure.hitsMax * 0.75);
            }
        });

        console.log(needingRepair.length);

        if(needingRepair.length < 1) {
            creep.memory.action = 'harvest';
            return;
        }

        creep.memory.action = 'fixing';
        console.log(needingRepair[0])
        creep.memory.fixing = needingRepair[0].id;
    },

    fix: function(creep) {
        let targetStructure = Game.getObjectById(creep.memory.fixing);
        let attempt = creep.repair(targetStructure);

        switch (attempt) {
            case ERR_INVALID_TARGET:
                creep.memory.action = 'choosing';
                break;
            case ERR_NOT_IN_RANGE:
                creep.moveTo(targetStructure);
                break;
            case ERR_NOT_ENOUGH_RESOURCES:
                creep.memory.action = 'harvest';
                break;
            case OK:
                creep.memory.fixing = null;
                break;
            default:
                console.log(creep.name + "Repair attempt returned code: " + attempt);
                break;
        }

        return;
    }
};

module.exports = roleMaintainer;
