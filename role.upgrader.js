var roleUpgrader = {
    name: 'Upgrader',
    desiredNumber: 6,
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
        if(creep.memory.upgrading && creep.store[RESOURCE_ENERGY] == 0) {
            creep.memory.upgrading = false;
            creep.say('🔄 harvest');
	    }
	    if(!creep.memory.upgrading && creep.store.getFreeCapacity() == 0) {
	        creep.memory.upgrading = true;
	        creep.say('⚡ upgrade');
	    }

	    if(creep.memory.upgrading) {
            if(creep.upgradeController(creep.room.controller) == ERR_NOT_IN_RANGE) {
                creep.moveTo(creep.room.controller, {visualizePathStyle: {stroke: '#ffffff'}});
            }
        }
        else {
	    let structureSources = creep.room.find(FIND_MY_STRUCTURES).filter(structure => {
		return (structure.structureType == STRUCTURE_STORAGE ||
			structure.structureType == STRUCTURE_CONTAINER) &&
			structure.store.getUsedCapacity(RESOURCE_ENERGY) > (300 * this.desiredNumber);
	    });
	    
	    structureSources.sort((a, b) => {
		return this.structurePriority.indexOf(a.structureType) 
		    - this.structurePriority.indexOf(b.structureType);
	    });

	    if (structureSources.length > 0) {
		let result = creep.withdraw(structureSources[0], RESOURCE_ENERGY);
		switch (result) {
		    case ERR_NOT_IN_RANGE:
			creep.moveTo(structureSources[0], {visualizePathStyle: {stroke: '#ffaa00'}});
			break;
		    default:
			console.log(result);
		}

		return;
	    }

            var sources = creep.room.find(FIND_SOURCES);
            if(creep.harvest(sources[0]) == ERR_NOT_IN_RANGE) {
                creep.moveTo(sources[0], {visualizePathStyle: {stroke: '#ffaa00'}});
            }
        }
    },

    // We always need upgraders
    shouldSpawn: function(room) {
        return true;
    }
};

module.exports = roleUpgrader;
