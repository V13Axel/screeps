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

    if(Object.keys(Game.creeps).length == 0) {
        Game.notify("Creeps are dead! I repeat, creeps are DEAD");
    }
};

function constructWithEnergyBudget(role, budget) {
    return role.definition;
    let parts = role.definition;
    let budgetFits = {};

    while(budgetFits.filter(item => item).length > 0) {
        console.log(JSON.stringify(budgetFits));
        for (var partName in role.partsBudgets) {
            if(!partName in budgetFits) {
                budgetFits[partName] = true;
            }

            let details = role.partsBudgets[partName];
            if(total + details.cost < budget) {
                parts.push(partName);
                continue;
            } 

            budgetFits[partName] = false;
        }
    }
    
    if(parts.length < 3) {
        return role.definition;
    }

    return parts; 
};

function spawnCreepsIfNeeded(activeSpawn, role, roleDetails, roomCreeps, room) {
    let desiredNumber = roleDetails.shouldSpawn(room) 
        ? roleDetails.desiredNumber 
        : 0;

    let creeps = Object.values(roomCreeps).filter(creep => creep.memory.role == role);
    let startedSpawning = false;

    creeps.forEach(creep => roleDetails.run(creep));

    if(
        !activeSpawn.spawning &&
        !startedSpawning &&
        creeps.length < desiredNumber &&
        activeSpawn.room.energyAvailable >= 300
    ) {
        let body = constructWithEnergyBudget(
            roleDetails,
            Math.max(activeSpawn.room.energyAvailable * .8, 300)
        );
        let name = role + Game.time;
        let memory = { memory: { role } };


        startedSpawning = true;
        activeSpawn.spawnCreep(
            body,
            name,
            memory
        );
    } else if (activeSpawn.spawning) {
        console.log("Attempting to build a creep: ", activeSpawn.spawning.name);
    }

    if(creeps.length > 0 && creeps.length > desiredNumber) {
        creeps[0].suicide();
    }
};

function roomLoop(room) {
    constructRoads(room);
    // console.log("Processing room " + id);
    let spawns = room.find(FIND_MY_SPAWNS);
    let roomCreeps = room.find(FIND_MY_CREEPS);

    for(var role in roles) {
        let roleDetails = roles[role];

        for (var spawn of spawns) {
            spawnCreepsIfNeeded(spawn, role, roleDetails, roomCreeps, room);
        }
    }
};

function constructRoads(room) {
    if(Game.flags[room.name + 'RoadStartFlag'] && Game.flags[room.name + 'RoadEndFlag']) {
        let path = room.findPath(
            Game.flags[room.name + 'RoadStartFlag'].pos,
            Game.flags[room.name + 'RoadEndFlag'].pos,
            {
                ignoreCreeps: true,
                ignoreRoads: true,
            }
        )
        createRoadConstructionSitesAlongPath(room, path);

        Game.flags[room.name + 'RoadStartFlag'].remove();
        Game.flags[room.name + 'RoadEndFlag'].remove();
        return;
    }

    if(room.memory.hasRoads) {
        return;
    }


    let spawn = room.find(FIND_MY_SPAWNS)[0];
    room.createFlag(
        spawn.pos,
        room.name + 'RoadStartFlag'
    );
    room.createFlag(
        room.controller.pos,
        room.name + 'RoadEndFlag'
    );

    room.memory.hasRoads = true;
}

function createRoadConstructionSitesAlongPath(room, path) {
    path.forEach(pos => {
        // room.visual.circle(pos.x, pos.y);
        room.getPositionAt(pos.x, pos.y).createConstructionSite(
            STRUCTURE_ROAD
        );
    });
}

module.exports.loop = function () {
    

    for (var [id, room] of Object.entries(Game.rooms)) {
        roomLoop(room);
    }

    cleanupDeadCreeps();
}
