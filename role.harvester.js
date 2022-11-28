var roleHarvester = {
    name: 'Harvester',
    desiredNumber: 4,
    definition: [WORK, CARRY, MOVE],
    partsBudgets: {
        [WORK]: {
            costModifier: .5,
            cost: 100,
            limit: 5,
        },
        [CARRY]: {
            costModifier: .5,
            cost: 50
        },
    },

    structurePriority: [
        STRUCTURE_SPAWN,
        STRUCTURE_EXTENSION,
        STRUCTURE_TOWER,
	STRUCTURE_STORAGE,
	STRUCTURE_CONTAINER
    ],

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

    shouldSpawn: function(room) {
        let harvesterWorkPartsTotal = room.find(FIND_MY_CREEPS, {
            filter: (creep) => {
                return (creep.memory.role == 'harvester');
            }
        }).reduce((total, creep) => { 
            return total + creep.body.filter(part => part.type == WORK).length;
        }, 0);

        let numberOfSources = room.find(FIND_SOURCES).length;

        return harvesterWorkPartsTotal / numberOfSources < 5;
    },

    deliver: function(creep) {
        var targets = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return (structure.structureType == STRUCTURE_CONTAINER ||
                        structure.structureType == STRUCTURE_STORAGE ||
                        structure.structureType == STRUCTURE_EXTENSION) && 
                        structure.store.getFreeCapacity(RESOURCE_ENERGY) > 0;
            }
        });

        targets.sort((a, b) => {
            return this.structurePriority.indexOf(a.structureType)
                 - this.structurePriority.indexOf(b.structureType);
        });

        if(targets.length > 0) {
            let attempt = creep.transfer(targets[0], RESOURCE_ENERGY);

            switch (attempt) {
                case ERR_NOT_IN_RANGE:
                    creep.moveTo(targets[0], {visualizePathStyle: {stroke: '#ffffff'}});
                    break;
                case OK:
                    if(creep.store.getFreeCapacity() == 0) {
                        creep.memory.action = 'harvesting';
                    }
                    break;
                case ERR_NOT_ENOUGH_ENERGY:
                    creep.memory.action = 'harvesting';
                    break;
            }
        } else {
            console.log("No targets?");
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
