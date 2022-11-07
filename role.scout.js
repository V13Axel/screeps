var roleScout = {
    desiredNumber: 1,
    definition: [WORK, CARRY, MOVE],
    partsBudgets: {
        [WORK]: {
            costModifier: (1/3),
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

    shouldSpawn: function(room) {
        if(Game.flags['Scoutflag']) {
            return true;
        }

        return false;
    },

    /** @param {Creep} creep **/
    run: function(creep) {
        creep.say(creep.memory.action);
        switch(creep.memory.action) {
            case 'returning':
                this.goHome(creep);
                break;
            case 'scouting':
                this.scout(creep);
                break;
            default:
                creep.memory.action = 'returning';
        }
    },

    goHome: function(creep) {
        let activeSpawn = Game.spawns['Spawn1'];
        if(creep.pos.isNearTo(activeSpawn)) {
            creep.memory.action = 'scouting';
            return;
        }

        creep.moveTo(activeSpawn);
    },

    scout: function(creep) {
        let targetFlag = Game.flags['Scoutflag'];
        let activeSpawn = Game.spawns['Spawn1'];

        if(!targetFlag && !creep.pos.isNearTo(activeSpawn)) {
            creep.memory.action = 'returning';
            return;
        }

        if(!creep.pos.isNearTo(targetFlag)) {
            creep.moveTo(targetFlag);
        }
    }
};

module.exports = roleScout;
