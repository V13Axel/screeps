// class Source {
//     x = 0;
//     y = - 0;
//
//     // Just a list of names for now
//     creeps = [];
// }


var sourceDirector = {
    sourceIds : [],
    sourceCreepCounts: {},
    directedCreeps : [],

    fillSources(room) {
        this.sourceIds = room.find(FIND_SOURCES).map(source => source.id);
    },

    giveMeASource(creep) {
        if(this.directedCreeps.includes(creep.name)) {
            // This shouldn't happen ... but y'know. Just in case.
            return this.directedCreeps[creep.name];
        }

        this.sourceIds.sort((a, b) => this.sourceCreepCounts[a] > this.sourceCreepCounts[b])
        let source = this.sourceIds[0];

        creep.memory.source = source;
        this.directedCreeps.push(creep.name);
    },

    cleanUpCreep(creep) {
        let index = this.directedCreeps.indexOf(creep.name);

        delete this.directedCreeps[index];
    }
}

module.exports = sourceDirector;
