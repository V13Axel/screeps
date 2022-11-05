var roleHarvester = require('role.harvester');
var roleUpgrader = require('role.upgrader');
var roleBuilder = require('role.builder');

var sourceDirector = require('sourcedirector');

var roles = {
    'harvester': roleHarvester,
    'upgrader': roleUpgrader,
    'builder': roleBuilder,
}

var ticksSinceFilledDirector = 0;

function cleanupDeadCreeps() {
    let alive = Object.keys(Game.creeps);
    for (var name in Memory.creeps) {
        if(!alive.includes(name)) {
            delete Memory.creeps[name];
            console.log('Freed memory of ' + name);
        }
    }
}

module.exports.loop = function () {
    cleanupDeadCreeps();

    // ticksSinceFilledDirector++;
    // if(ticksSinceFilledDirector > 120) {
    //     // Temporary hack
    //     sourceDirector.fillSources();
    //
    //     ticksSinceFilledDirector = 0;
    // }

    for(var role in roles) {
        let roleDetails = roles[role];
        let desiredNumber = roleDetails.desiredNumber;
        let creeps = Object.values(Game.creeps).filter(creep => creep.memory.role == role);

        creeps.forEach(creep => roleDetails.run(creep));
        
        if(creeps.length < desiredNumber) {
            Game.spawns['Spawn1'].spawnCreep(
                roleDetails.definition,
                role + Game.time,
                { memory: { role } }
            )

            break;
        }

        if(creeps.length > desiredNumber) {
            creeps[0].suicide();
        }
    }
}
