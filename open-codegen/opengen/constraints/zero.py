import casadi.casadi as cs
import numpy as np
from .constraint import Constraint
import opengen.functions as fn


class Zero(Constraint):
    """
    A constraint to the zero set, that is 
    the set that contains only zero(es) ({0}).

    """

    def __init__(self):
        """
        Constructor for set Z = {0}

        """

    def distance_squared(self, u):
        return fn.norm2_squared(u)

    def project(self, u):
        raise NotImplementedError()

    def is_convex(self):
        return True

    def is_compact(self):
        return True
