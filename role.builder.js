var roleBuilder = {
    desiredNumber: 4,
    definition: [WORK, MOVE, CARRY],
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
		console.log('here');
	    }
	}
	else {
	    var sources = creep.room.find(FIND_SOURCES);
	    if(creep.harvest(sources[1]) == ERR_NOT_IN_RANGE) {
		creep.moveTo(sources[1], {visualizePathStyle: {stroke: '#ffaa00'}});
	    }
	}
    },
};

module.exports = roleBuilder;
