var roleBuilder = {
    name: 'Builder',
    desiredNumber: 4,
    definition: [WORK, MOVE, CARRY],
    partsBudgets: {
        [WORK]: {
            costModifier: .5,
            cost: 100
        },
        [CARRY]: {
            costModifier: .5,
            cost: 50
        },
    },

    structurePriority: [
	STRUCTURE_STORAGE,
	STRUCTURE_CONTAINER
    ],

    shouldSpawn(room) {
	return room.find(FIND_CONSTRUCTION_SITES).length > 0;
    },

    /** @param {Creep} creep **/
    run: function(creep) {
	if(creep.memory.building && creep.store[RESOURCE_ENERGY] == 0) {
	    creep.memory.building = false;
	    creep.say('🔄 harvest');
	}
	if(!creep.memory.building && creep.store.getFreeCapacity() == 0) {
	    creep.memory.building = true;
	    creep.say('🚧 build');
	}

	if(creep.memory.building) {
	    var targets = creep.room.find(FIND_CONSTRUCTION_SITES);
	    if(targets.length) {
		if(creep.build(targets[0]) == ERR_NOT_IN_RANGE) {
		    creep.moveTo(targets[0], {visualizePathStyle: {stroke: '#ffffff'}});
		}
	    } else {
		let attempt = creep.moveTo(Game.spawns['Spawn1'], {vizualizePathStyle: {stroke: '#fefefe'}});
	    }
	}
	else {
	    let structureSources = creep.room.find(FIND_STRUCTURES).filter(structure => {
		switch(structure.structureType) {
		    case STRUCTURE_STORAGE:
			// console.log("Structure is storage", structure.store.getUsedCapacity(RESOURCE_ENERGY));
			return structure.store.getUsedCapacity(RESOURCE_ENERGY) > 3000;
		    case STRUCTURE_CONTAINER:
			// console.log("Structure is container", structure.store.getUsedCapacity(RESOURCE_ENERGY));
			return structure.store.getUsedCapacity(RESOURCE_ENERGY) > 0;
		    default:
			return false;
		}
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

	    // Only if structure sources are all depleted

	    let sources = creep.room.find(FIND_SOURCES);

	    if(creep.harvest(sources[0]) == ERR_NOT_IN_RANGE) {
		creep.moveTo(sources[0], {visualizePathStyle: {stroke: '#ffaa00'}});
	    }
	}
    },
};

module.exports = roleBuilder;
