var roleDefenseTech = {
    name: 'DefenseTech',
    desiredNumber: 1,
    definition: [WORK, CARRY, MOVE],
    partsBudgets: {
        [CARRY]: {
            costModifier: 1,
            cost: 50
        },
    },

    /** @param {Creep} creep **/
    run: function(creep) {
        if(Memory.creepsAnnounce) {
            this.announce(creep, creep.memory.action);
        }
        switch(creep.memory.action) {
            case 'refilling':
                this.refill(creep);
                break;
            case 'harvest':
                this.harvest(creep);
                break;
            case 'waiting':
                this.wait(creep);
                break;
            default:
                creep.memory.action = 'waiting';
        }

    },

    // For now, always spawn maintainers.
    // Future, though? Maybe some kind of smart approach that
    // Only spawns when something in the room is <50% health?
    shouldSpawn: function(room) {
        return room.find(FIND_STRUCTURES, {
            filter: (structure) => structure.structureType == STRUCTURE_TOWER
        }).length;
    },

    harvest: function(creep) {
        if(creep.store.getFreeCapacity() == 0) {
            this.announce(creep, 'wait');
            creep.memory.action = 'waiting';
            return;
        }

        let structureSources = creep.room.find(FIND_STRUCTURES).filter(structure => {
            switch(structure.structureType) {
                case STRUCTURE_STORAGE:
                    // console.log("Structure is storage", structure.store.getUsedCapacity(RESOURCE_ENERGY));
                    return structure.store.getUsedCapacity(RESOURCE_ENERGY) > 0;
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
            }

            return;
        }

        if(!creep.memory.source) {
            let sources = creep.room.find(FIND_SOURCES);

            creep.memory.source = sources[Math.floor(Math.random()*sources.length)].id;
        }

        let target = Game.getObjectById(creep.memory.source);
        let attempt = creep.harvest(target); 
        switch (attempt) {
            case ERR_NOT_IN_RANGE:
                creep.moveTo(target, {visualizePathStyle: {stroke: '#ffaa00'}});
                break;
            case ERR_INVALID_TARGET:
                creep.memory.source = null;
                break;
        }
    },

    wait: function(creep) {
        let towers = creep.room.find(FIND_STRUCTURES, {
            filter: (structure) => {
                return structure.structureType == STRUCTURE_TOWER;
            }
        });
        creep.moveTo(towers[0]);

        if(towers.length < 1 && creep.store.getFreeCapacity() > 0) {
            this.announce(creep, 'harvesting');
            creep.memory.action = 'harvest';
            return;
        }

        if(towers.length < 1 && creep.store.getFreeCapacity() > 0) {
            this.announce(creep, 'harvesting');
            creep.memory.action = 'harvest';
            return;
        }

        if(towers.length < 1) {
            return;
        }

        this.announce(creep, 'refilling');
        creep.memory.action = 'refilling';
        creep.memory.refilling = towers[0].id;
    },

    refill: function(creep) {
        let targetStructure = Game.getObjectById(creep.memory.refilling);
        let attempt = creep.transfer(targetStructure, RESOURCE_ENERGY);

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
            case ERR_FULL:
                this.announce(creep, 'waiting');
                creep.memory.action = 'waiting';
                break;
            case OK:
                if(targetStructure.hits > targetStructure.hitsMax * 0.95) {
                    creep.memory.refilling = null;
                }
                break;
            default:
                console.log(creep.name + "Fill attempt returned code: " + attempt);
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

module.exports = roleDefenseTech;
