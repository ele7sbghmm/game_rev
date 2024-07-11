def euler2mat() -> list[list[float]]
    return [[
        ch * ca,
        sh * sb       -  ch * sa * cb,
        ch * sa * sb  +  sh * cb,
    ], [
        sa,
        ca * cb,
        -ca * sb,
    ], [
        -sh * ca,
        sh * sa * cb  +  ch * sb,
        -sh * sa * sb  +  ch * cb,
    ]]

