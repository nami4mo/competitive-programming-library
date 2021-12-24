''' [円の交点] '''
def get_circles_cross_points(x1,y1,r1,x2,y2,r2):
    x3=x2-x1
    y3=y2-y1
    a=(x3**2+y3**2+r1**2-r2**2)/2
    root=(x3**2+y3**2)*(r1**2)-a**2
    if root<0: return None,None,None,None
    root=root**0.5
    res_x1=(a*x3+y3*root)/(x3**2+y3**2)
    res_y1=(a*y3-x3*root)/(x3**2+y3**2)
    res_x2=(a*x3-y3*root)/(x3**2+y3**2)
    res_y2=(a*y3+x3*root)/(x3**2+y3**2)
    return res_x1+x1,res_y1+y1,res_x2+x1,res_y2+y1