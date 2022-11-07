const roleHarvester = require('role.harvester');
const roleUpgrader = require('role.upgrader');
const roleBuilder = require('role.builder');
const roleMaintainer = require('./role.maintainer');
const roleScout = require('./role.scout');

var roles = {
    'harvester': roleHarvester,
    'upgrader': roleUpgrader,
    'builder': roleBuilder,
    'maintainer': roleMaintainer,
    'scout': roleScout,
}


function cleanupDeadCreeps() {
    let alive = Object.keys(Game.creeps);
    for (var name in Memory.creeps) {
        if(!alive.includes(name)) {
            delete Memory.creeps[name];
            console.log('Freed memory of ' + name);
        }
    }
};

function constructWithEnergyBudget(role, budget) {
    return role.definition;
    let parts = [];
    
    for (var partName in role.partsBudgets) {
        let details = role.partsBudgets[partName];
        let count = Math.floor((details.costModifier * budget) / details.cost);

        for(let i = 0; i < count; i++) {
            parts.push(partName);
        }
    }

    if(parts.length < 3) {
        return role.definition;
    }

    return parts; 
};

module.exports.loop = function () {
    cleanupDeadCreeps();
    for (var [id, room] of Object.entries(Game.rooms)) {
        // console.log("Processing room " + id);
        let activeSpawn = room.find(FIND_MY_SPAWNS)[0];
        let roomCreeps = room.find(FIND_MY_CREEPS);

        for(var role in roles) {
            let roleDetails = roles[role];
            let desiredNumber = roleDetails.shouldSpawn(room) ? roleDetails.desiredNumber : 0;
            let creeps = Object.values(roomCreeps).filter(creep => creep.memory.role == role);

            creeps.forEach(creep => roleDetails.run(creep));
            
            if(activeSpawn) {
                if(creeps.length < desiredNumber && activeSpawn.room.energyAvailable >= 300) {
                    let body = constructWithEnergyBudget(
                            roleDetails,
                            Math.max(activeSpawn.room.energyAvailable * .8, 300)
                        );
                    let name = role + Game.time;
                    let memory = { memory: { role } };

                    console.log("Attempting to build a creep: ", body, name);

                    activeSpawn.spawnCreep(
                        body,
                        name,
                        memory
                    );
                }

                if(creeps.length > 0 && creeps.length > desiredNumber) {
                    creeps[0].suicide();
                }
            }
        }
    }
}
