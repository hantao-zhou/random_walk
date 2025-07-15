#include <emscripten/bind.h>
#include <vector>
#include <map>
#include <deque>
#include <random>
#include <utility>

using namespace emscripten;

struct WalkResult {
    std::vector<float> x;
    std::vector<float> y;
};

static std::pair<std::pair<int,int>, std::pair<int,int>> edge_key(std::pair<int,int> a, std::pair<int,int> b) {
    if (a <= b) return {a,b};
    return {b,a};
}

WalkResult random_walk(int steps) {
    std::mt19937 rng(std::random_device{}());
    std::uniform_int_distribution<int> dist(0,3);
    WalkResult res;
    res.x.reserve(steps+1);
    res.y.reserve(steps+1);
    float cx=0, cy=0;
    res.x.push_back(cx);
    res.y.push_back(cy);
    for (int i=0;i<steps;i++) {
        int dir = dist(rng);
        switch(dir) {
            case 0: cx += 1; break;
            case 1: cx -= 1; break;
            case 2: cy += 1; break;
            default: cy -= 1; break;
        }
        res.x.push_back(cx);
        res.y.push_back(cy);
    }
    return res;
}

WalkResult erw_walk(int steps, double strength, int delay, int memory) {
    std::mt19937 rng(std::random_device{}());
    WalkResult res;
    res.x.reserve(steps+1);
    res.y.reserve(steps+1);
    int cx=0, cy=0;
    res.x.push_back(cx);
    res.y.push_back(cy);
    std::map<std::pair<std::pair<int,int>, std::pair<int,int>>, unsigned> counts;
    std::deque<std::pair<std::pair<int,int>, std::pair<int,int>>> history;

    for(int step=0; step<steps; ++step) {
        std::pair<int,int> neighbors[4] = {
            {cx+1, cy}, {cx-1, cy}, {cx, cy+1}, {cx, cy-1}
        };
        double weights[4];
        double total=0;
        for(int i=0;i<4;i++) {
            auto key=edge_key({cx,cy}, neighbors[i]);
            double count = counts[key];
            double w = (step >= delay) ? 1.0 + strength*count : 1.0;
            weights[i]=w;
            total += w;
        }
        double r = std::uniform_real_distribution<>(0,total)(rng);
        double acc=0; int choice=0;
        for(int i=0;i<4;i++) {
            acc += weights[i];
            if(r <= acc) { choice=i; break; }
        }
        auto next = neighbors[choice];
        auto key=edge_key({cx,cy}, next);
        counts[key] += 1;
        history.push_back(key);
        if(memory>0 && history.size()>static_cast<size_t>(memory)) {
            auto old = history.front();
            history.pop_front();
            auto it = counts.find(old);
            if(it!=counts.end() && it->second>0) it->second -= 1;
        }
        cx = next.first; cy = next.second;
        res.x.push_back(static_cast<float>(cx));
        res.y.push_back(static_cast<float>(cy));
    }
    return res;
}

double monte_carlo_erw_et(int trials, int steps, double strength, int delay, int memory) {
    std::mt19937 rng(std::random_device{}());
    unsigned long long total=0;
    for(int t=0;t<trials;t++) {
        std::map<std::pair<std::pair<int,int>, std::pair<int,int>>, unsigned> counts;
        std::deque<std::pair<std::pair<int,int>, std::pair<int,int>>> history;
        int cx=0, cy=0;
        for(int step=1; step<=steps; ++step) {
            std::pair<int,int> neighbors[4] = {
                {cx+1,cy}, {cx-1,cy}, {cx,cy+1}, {cx,cy-1}
            };
            double weights[4];
            double tot=0;
            for(int i=0;i<4;i++) {
                auto key=edge_key({cx,cy}, neighbors[i]);
                double count = counts[key];
                double w = (step>delay) ? 1.0 + strength*count : 1.0;
                weights[i]=w; tot+=w;
            }
            double r = std::uniform_real_distribution<>(0,tot)(rng);
            double acc=0; int choice=0;
            for(int i=0;i<4;i++){ acc+=weights[i]; if(r<=acc){choice=i; break;} }
            auto next = neighbors[choice];
            auto key=edge_key({cx,cy}, next);
            counts[key]+=1; history.push_back(key);
            if(memory>0 && history.size()>static_cast<size_t>(memory)) {
                auto old=history.front(); history.pop_front();
                auto it=counts.find(old); if(it!=counts.end() && it->second>0) it->second-=1;
            }
            cx=next.first; cy=next.second;
            if(cx==0 && cy==0){ total += step; break; }
            if(step==steps) total += steps;
        }
    }
    return static_cast<double>(total)/trials;
}

double monte_carlo_et(int trials, int steps) {
    std::mt19937 rng(std::random_device{}());
    unsigned long long total=0;
    for(int t=0;t<trials;t++) {
        int cx=0, cy=0;
        for(int step=1; step<=steps; ++step) {
            int dir = std::uniform_int_distribution<int>(0,3)(rng);
            if(dir==0) cx +=1; else if(dir==1) cx -=1; else if(dir==2) cy +=1; else cy -=1;
            if(cx==0 && cy==0){ total += step; break; }
            if(step==steps) total += steps;
        }
    }
    return static_cast<double>(total)/trials;
}

EMSCRIPTEN_BINDINGS(walk_module) {
    value_object<WalkResult>("WalkResult")
        .field("x", &WalkResult::x)
        .field("y", &WalkResult::y);
    register_vector<float>("VecF32");

    function("random_walk", &random_walk);
    function("erw_walk", &erw_walk);
    function("monte_carlo_erw_et", &monte_carlo_erw_et);
    function("monte_carlo_et", &monte_carlo_et);
}

