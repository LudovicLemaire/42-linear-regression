import math
import matplotlib.pyplot as plt

def	normalizeData(mileages, prices):
	x = []
	y = []
	minM = min(mileages)
	maxM = max(mileages)
	for mileage in mileages:
		x.append((mileage - minM) / (maxM - minM))
	minP = min(prices)
	maxP = max(prices)
	for price in prices:
		y.append((price - minP) / (maxP - minP))
	return (x, y)
	
def	normalizeElem(list, elem):
	return ((elem - min(list)) / (max(list) - min(list)))

def	denormalizeElem(list, elem):
	return ((elem * (max(list) - min(list))) + min(list))
	
def	gradientDescent(mileages, prices, learningRate, iterations):
	lossHistory = []
	t0History = [0.0]
	t1History = [0.0]
	t0 = 0.0
	t1 = 0.0
	message = "max epoch reached"
	
	for iteration in range(iterations):
		dt0 = 0
		dt1 = 0
		for mileage, price in zip(mileages, prices):
			dt0 += (t1 * mileage + t0) - price
			dt1 += ((t1 * mileage + t0) - price) * mileage
		t0 -= dt0 / len(mileages) * learningRate
		t1 -= dt1 / len(prices) * learningRate
		loss = lossFunction(t0, t1, mileages, prices)
		if iteration % 10 == 0:
			print("epoch {} - loss: {:.8}".format(iteration, loss))
		t0, t1, learningRate = boldDriver(loss, lossHistory, t0, t1, dt0, dt1, learningRate, len(mileages))
		lossHistory.append(loss)
		t0History.append(t0)
		t1History.append(t1)
		if earlyStopping(lossHistory):
			message = "early stopped"
			break
	print("\nend: {}.".format(message))
	print("epoch {} - loss: {:.8}".format(iteration, loss))
	return (t0, t1, lossHistory, t0History, t1History)
	
def	lossFunction(t0, t1, mileages, prices):
	loss = 0.0
	for mileage, price in zip(mileages, prices):
		loss += (price - (t1 * mileage + t0)) ** 2
	return (loss / len(mileages))

def	boldDriver(loss, lossHistory, t0, t1, dt0, dt1, learningRate, length):
	newLearningRate = learningRate
	if len(lossHistory) > 1:
		if loss >= lossHistory[-1]:
			t0 += dt0 / length * learningRate
			t1 += dt1 / length * learningRate
			newLearningRate *=  0.5
		else:
			newLearningRate *= 1.05
	return (t0, t1, newLearningRate)

def	earlyStopping(lossHistory):
	check = 8
	if len(lossHistory) > check:
		mean = sum(lossHistory[-(check):]) / check
		last = lossHistory[-1]
		if round(mean, 9) == round(last, 9): 
			return True
	return False

def	displayPlot(t0, t1, mileages, prices, lossHistory, t0History, t1History):
	lineX = [float(min(mileages)), float(max(mileages))]
	lineY = []
	for elem in lineX:
		elem = t1 * normalizeElem(mileages, elem) + t0
		lineY.append(denormalizeElem(prices, elem))
		
	plt.figure(1)
	plt.plot(mileages, prices, 'bo', lineX, lineY, 'r-')
	plt.xlabel('mileage')
	plt.ylabel('price')
	plt.show()
	
def	main():
	learningRate = 0.5
	iterations = 500
	mileages = [240000,139800,150500,185530,176000,114800,166800,89000,144500,84000,82029,63060,74000,97500]
	prices = [3650,3800,4400,4450,5250,5350,5800,5990,5999,6200,6390,6390,6600,6800]
	x, y = normalizeData(mileages, prices)
	t0, t1, lossHistory, t0History, t1History = gradientDescent(x, y, learningRate, iterations)
	displayPlot(t0, t1, mileages, prices, lossHistory, t0History, t1History)
	
if	__name__ == '__main__':
	main()

