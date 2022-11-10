var roleMaintainer = {
    name: 'Maintainer',
    desiredNumber: 2,
    definition: [WORK, CARRY, MOVE],
    partsBudgets: {
        [WORK]: {
            costModifier: .2,
            cost: 100
        },
        [CARRY]: {
            costModifier: .25,
            cost: 50
        },
        [MOVE]: {
            costModifier: .45,
            cost: 50
        },
    },

    /** @param {Creep} creep **/
    run: function(creep) {
        if(Memory.creepsAnnounce) {
            this.announce(creep, creep.memory.action);
        }
        switch(creep.memory.action) {
            case 'refill':
                this.refill(creep);
                break;
            case 'cleanup':
                this.cleanup(creep);
                break;
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
                creep.memory.action = 'cleanup';
        }

    },

    // For now, always spawn maintainers.
    // Future, though? Maybe some kind of smart approach that
    // Only spawns when something in the room is <50% health?
    shouldSpawn: function(room) {
        return true;
        return room.find(FIND_STRUCTURES).filter(structure => structure.hits < (structure.hitsMax / 2)).length;
    },

    refill(creep) {
        if(creep.store.getUsedCapacity() == 0 || !creep.memory.target) {
            console.log("uh what",
                creep.store.getUsedCapacity(),
                creep.memory.target
            );
            this.announce(creep, 'harvest');
            creep.memory.action = 'harvest';
            return;
        }

        let target = Game.getObjectById(creep.memory.target);
        let attempt = creep.transfer(target, RESOURCE_ENERGY);
        switch (attempt) {
            case ERR_NOT_IN_RANGE:
                creep.moveTo(target);
                break;
            case ERR_INVALID_TARGET:
                creep.memory.target = null;
                creep.memory.action = 'wait';
                break;
            case ERR_FULL:
                creep.memory.target = null;
                break;
            case OK:
                break;
            default:
                console.log("Attempting to refill extension resulted in", attempt);
        }
    },

    cleanup: function(creep) {
        let dropped = creep.room.find(FIND_DROPPED_RESOURCES);
        if(!dropped.length) {
            this.announce(creep, 'wait');
            creep.memory.action = 'waiting';
            return;
        }

        if(creep.pickup(dropped[0]) == ERR_NOT_IN_RANGE) {
            creep.moveTo(dropped[0]);
        }
    },

    harvest: function(creep) {
        if(creep.store.getFreeCapacity() == 0) {
            this.announce(creep, 'wait');
            creep.memory.action = 'waiting';
            return;
        }

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
    },

    _refill_extensions(creep) {
        // Refill extensions
        let extensions = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return structure.structureType == STRUCTURE_EXTENSION
                    && structure.store.getFreeCapacity(RESOURCE_ENERGY) > 0;
            }
        });

        if(extensions.length > 0) {
            this.announce(creep, 'refilling');
            creep.memory.action = 'refill';
            creep.memory.target = extensions[0].id;

            return true;
        }

        return false;
    },

    _refill_towers(creep) {
        // Refill towers
        let towers = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return structure.structureType == STRUCTURE_TOWER
                    && structure.store.getFreeCapacity() > 200;
            }
        });

        if(towers.length > 0) {
            this.announce(creep, 'harvesting');
            creep.memory.action = 'harvest';
            creep.memory.target = towers[0].id;
            return true;
        }

        return false;
    },
    
    _repair_structures(creep) {
        // Repair structures
        var needingRepair = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return (structure.structureType == STRUCTURE_ROAD ||
                        structure.structureType == STRUCTURE_WALL ||
                        structure.structureType == STRUCTURE_RAMPART) && 
                        (structure.hits < structure.hitsMax * 0.75);
            }
        });

        if(needingRepair.length > 0) {
            this.announce(creep, 'fixing');
            creep.memory.action = 'fixing';
            creep.memory.fixing = needingRepair[0].id;
            return true;
        }

        return false;
    },

    _idle(creep) {
        // Nothing else to do? Just head towards a spawn I guess.
        let spawns = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return structure.structureType == STRUCTURE_SPAWN
            }
        });
        creep.moveTo(spawns[0]);
    },

    wait: function(creep) {
        // Harvest if we're empty
        if(creep.store.getFreeCapacity() > 0) {
            this.announce(creep, 'harvesting');
            creep.memory.action = 'harvest';
            return;
        }

        if(this._refill_extensions(creep)) {
            // console.log("Refilling extension");
            return;
        }

        if(this._refill_towers(creep)) {
            // console.log("Refilling towers");
            return;
        }

        if(this._repair_structures(creep)) {
            // console.log("Repairing structures");
            return;
        }

        // console.log("Idle time");
        this._idle(creep);
    },

    fix: function(creep) {
        let targetStructure = Game.getObjectById(creep.memory.fixing);
        let attempt = creep.repair(targetStructure);

        switch (attempt) {
            case ERR_INVALID_TARGET:
                this.announce(creep, 'waiting');
                creep.memory.action = 'waiting';
                break;
            case ERR_NOT_IN_RANGE:
                creep.moveTo(targetStructure);
                break;
            case ERR_NOT_ENOUGH_RESOURCES:
                this.announce(creep, 'harvesting');
                creep.memory.action = 'harvest';
                break;
            case OK:
                if(targetStructure.hits > targetStructure.hitsMax * 0.95) {
                    creep.memory.fixing = null;
                }
                break;
            default:
                console.log(creep.name + "Repair attempt returned code: " + attempt);
                break;
        }

        return;
    },

    announce: function(creep, string) {
        if(Memory.creepsAnnounce) {
            creep.say(string);
        }
    }
};

module.exports = roleMaintainer;
