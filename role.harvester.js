var roleHarvester = {
    desiredNumber: 4,
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
            case 'harvesting':
                this.harvest(creep);
                break;
            case 'deliver':
                this.deliver(creep);
                break;
            default:
                creep.memory.action = 'harvesting';
                break;
        }
    },

    // **always** spawn harvesters.
    shouldSpawn: function(room) {
        return true;
    },

    deliver: function(creep) {
        var targets = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return (structure.structureType == STRUCTURE_EXTENSION ||
                        structure.structureType == STRUCTURE_SPAWN ||
                        structure.structureType == STRUCTURE_CONTAINER ||
                        structure.structureType == STRUCTURE_TOWER) && 
                        structure.store.getFreeCapacity(RESOURCE_ENERGY) > 0;
            }
        });

        if(targets.length > 0) {
            let attempt = creep.transfer(targets[0], RESOURCE_ENERGY);

            switch (attempt) {
                case ERR_NOT_IN_RANGE:
                    creep.moveTo(targets[0], {visualizePathStyle: {stroke: '#ffffff'}});
                    break;
                case OK:
                    creep.memory.action = 'harvesting';
                    break;
            }
        } else {
            creep.moveTo(Game.spawns['Spawn1']);
        }
    },

    harvest: function(creep) {
        if(creep.store.getFreeCapacity() > 0) {
            var sources = creep.room.find(FIND_SOURCES);
            if(creep.harvest(sources[1]) == ERR_NOT_IN_RANGE) {
                creep.moveTo(sources[1], {visualizePathStyle: {stroke: '#ffaa00'}});
            }
        } else {
            creep.memory.action = 'deliver'
        }
    }
};

module.exports = roleHarvester;
