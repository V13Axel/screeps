var roleHarvester = require('role.harvester');
var roleUpgrader = require('role.upgrader');
var roleBuilder = require('role.builder');
const roleMaintainer = require('./role.maintainer');

var roles = {
    'harvester': roleHarvester,
    'upgrader': roleUpgrader,
    'builder': roleBuilder,
    'maintainer': roleMaintainer,
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
    let activeSpawn = Game.spawns['Spawn1'];

    for(var role in roles) {
        let roleDetails = roles[role];
        let desiredNumber = roleDetails.desiredNumber;
        let creeps = Object.values(Game.creeps).filter(creep => creep.memory.role == role);

        creeps.forEach(creep => roleDetails.run(creep));
        
        if(creeps.length < desiredNumber && activeSpawn.room.energyAvailable > 300) {
            let body = constructWithEnergyBudget(
                    roleDetails,
                    (activeSpawn.room.energyAvailable * .8)
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

        if(creeps.length > desiredNumber) {
            creeps[0].suicide();
        }
    }
}
