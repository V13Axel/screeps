var roleMaintainer = {
    name: 'Maintainer',
    desiredNumber: 1,
    definition: [CARRY, CARRY],
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
            case 'refill':
                this.refill(creep);
                break;
            case 'collect':
                this.collect(creep);
                break;
            case 'waiting':
                this.wait(creep);
                break;
            default:
                creep.memory.action = 'waiting';
        }

    },

    // For now, always spawn maintainers.
    shouldSpawn: function(_) {
        return true;
    },

    refill(creep) {
        if(creep.store.getUsedCapacity() == 0 || !creep.memory.target) {
            this.announce(creep, 'collect');
            creep.memory.action = 'collect';
            return;
        }

        let target = Game.getObjectById(creep.memory.target);
        let attempt = creep.transfer(target, RESOURCE_ENERGY);
        switch (attempt) {
            case ERR_NOT_IN_RANGE:
                creep.moveTo(target);
                break;
            case ERR_INVALID_TARGET:
            case ERR_FULL:
                this._refill_extensions(creep);
                break;
            case OK:
                break;
            default:
                console.log("Attempting to refill extension resulted in", attempt);
        }
    },

    collect: function(creep) {
        if(creep.store.getFreeCapacity() == 0) {
            this.announce(creep, 'wait');
            creep.memory.action = 'waiting';
            return;
        }

        let structureSources = creep.room.find(FIND_MY_STRUCTURES).filter(structure => {
            return (structure.structureType == STRUCTURE_STORAGE ||
                structure.structureType == STRUCTURE_SPAWN ||
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
                    console.log(creep.name, " withdraw result ", result);
            }

            return;
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

    // _refill_towers(creep) {
    //     // Refill towers
    //     let towers = creep.room.find(FIND_STRUCTURES, {
    //         filter: (structure) => {
    //             return structure.structureType == STRUCTURE_TOWER
    //                 && structure.store.getFreeCapacity() > 200;
    //         }
    //     });
    //
    //     if(towers.length > 0) {
    //         this.announce(creep, 'collecting');
    //         creep.memory.action = 'collect';
    //         creep.memory.target = towers[0].id;
    //         return true;
    //     }
    //
    //     return false;
    // },

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
        // collect if we're empty
        if(creep.store.getFreeCapacity() > 0) {
            this.announce(creep, 'collecting');
            creep.memory.action = 'collect';
            return;
        }

        if(this._refill_extensions(creep)) {
            return;
        }

        // if(this._refill_towers(creep)) {
        //     return;
        // }

        // console.log("Idle time");
        this._idle(creep);
    },

    announce: function(creep, string) {
        if(Memory.creepsAnnounce) {
            creep.say(string);
        }
    }
};

module.exports = roleMaintainer;
