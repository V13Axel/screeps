const roleHarvester = require('role.harvester');
const roleUpgrader = require('role.upgrader');
const roleBuilder = require('role.builder');
const roleMaintainer = require('./role.maintainer');
const roleScout = require('./role.scout');
const roleDefenseTech = require('./role.defenseTech');

const constructionManager = require('./manager.construction');
const towerManager = require('./building.tower');
const spawnManager = require('./manager.spawns');

var roles = {
    'harvester': roleHarvester,
    'upgrader': roleUpgrader,
    'builder': roleBuilder,
    'maintainer': roleMaintainer,
    'scout': roleScout,
    'defenseTech': roleDefenseTech,
}


function cleanupDeadCreeps() {
    let alive = Object.keys(Game.creeps);
    for (var name in Memory.creeps) {
        if(!alive.includes(name)) {
            delete Memory.creeps[name];
            console.log('Freed memory of ' + name);
        }
    }

    if(Object.keys(Game.creeps).length == 0) {
        Game.notify("Creeps are dead! I repeat, creeps are DEAD");
    }
};


function roomLoop(room) {
    constructionManager.constructRoads(room);
    towerManager.run(room);

    let spawns = room.find(FIND_MY_SPAWNS);
    let roomCreeps = room.find(FIND_MY_CREEPS);

    for(var role in roles) {
        let roleDetails = roles[role];

        for (var activeSpawn of spawns) {
            let desiredNumber = roleDetails.shouldSpawn(room) 
                ? roleDetails.desiredNumber 
                : 0;

            let creeps = Object.values(roomCreeps).filter(creep => creep.memory.role == role);
            let startedSpawning = false;

            creeps.forEach(creep => {
                let energyOnGround = creep.pos.lookFor(LOOK_ENERGY)
                if(energyOnGround.length > 0) {
                    creep.pickup(energyOnGround[0]);
                }
                roleDetails.run(creep)
            });

            startedSpawning = spawnManager.spawnIfNecessary(activeSpawn, creeps, desiredNumber, roleDetails);

            if(creeps.length > 0 && creeps.length > desiredNumber) {
                creeps[0].suicide();
            }
        }
    }
};


module.exports.loop = function () {
    for (var [id, room] of Object.entries(Game.rooms)) {
        roomLoop(room);
    }

    cleanupDeadCreeps();
}
